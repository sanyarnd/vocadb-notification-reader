use std::time::Duration;

use actix_web::cookie::Cookie;
use actix_web::http::Method;
use anyhow::Context;
use log::{debug, info};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::errors::Result;
use crate::client::errors::VocadbClientError;
use crate::client::models::misc::PartialFindResult;
use crate::client::models::query::{LanguagePreference, OptionalFields};
use crate::client::models::song::SongForApiContract;
use crate::client::models::user::{Inbox, UserForApiContract, UserMessageContract};

pub struct Client<'a> {
    pub client: awc::Client,
    pub base_url: &'a str,
    pub cookies: Vec<Cookie<'a>>,
}

impl<'a> Client<'a> {
    pub fn vocadb(cookies: &'a [String]) -> Result<Client<'a>> {
        Client::new("https://vocadb.net", cookies)
    }

    pub fn touhoudb(cookies: &'a [String]) -> Result<Client<'a>> {
        Client::new("https://touhoudb.com", cookies)
    }

    pub fn utaitedb(cookies: &'a [String]) -> Result<Client<'a>> {
        Client::new("https://utaitedb.net", cookies)
    }

    fn new(base_url: &'a str, cookies: &'a [String]) -> Result<Client<'a>> {
        let mut parsed_cookies = vec![];
        for c in cookies {
            let cookie = Cookie::parse(c).context("Unable to parse a cookie")?;
            parsed_cookies.push(cookie)
        }

        let awc_client = awc::Client::builder()
            .connector(awc::Connector::new().timeout(Duration::from_secs(30)))
            .timeout(Duration::from_secs(30))
            .max_redirects(0)
            .finish();

        Ok(Client {
            client: awc_client,
            base_url,
            cookies: parsed_cookies,
        })
    }

    fn add_cookie(&mut self, cookie: &Cookie<'a>) -> &Client {
        self.cookies.push(cookie.clone());

        self
    }

    fn clear_cookie(&mut self) -> &Client {
        self.cookies.clear();

        self
    }

    fn create_request(&self, url: &String, method: Method) -> awc::ClientRequest {
        let mut builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::DELETE => self.client.delete(url),
            _ => panic!("Unsupported method"),
        };

        for cookie in &self.cookies {
            builder = builder.cookie(cookie.clone());
        }

        builder
    }

    async fn http_get<T, R>(&self, url: &String, query: &T) -> Result<R>
    where
        R: DeserializeOwned,
        T: Serialize,
    {
        let request = self.create_request(url, Method::GET);
        debug!("Sending GET request {:?}", request);
        let body = request
            .query(query)
            .context("Unable to construct a query")?
            .send()
            .await?
            .body()
            .await?;
        debug!("Response: {}", String::from_utf8(body.to_vec()).unwrap());
        let json = serde_json::from_slice(&body).context("Unable to deserialize a payload")?;

        Ok(json)
    }

    async fn http_delete_void<T>(&self, url: &String, query: &T) -> Result<()>
    where
        T: Serialize,
    {
        let request = self.create_request(url, Method::DELETE);
        request
            .query(query)
            .context("Unable to construct a query")?
            .send()
            .await?;

        Ok(())
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        #[derive(Serialize, Debug)]
        struct LoginFormData<'a> {
            #[serde(rename = "UserName")]
            username: &'a str,
            #[serde(rename = "Password")]
            password: &'a str,
            #[serde(rename = "KeepLoggedIn")]
            keep_logged_in: bool,
        }

        let login_data = LoginFormData {
            username,
            password,
            keep_logged_in: true,
        };

        info!("Logging user {}", username);

        let response = self
            .client
            .post(format!("{}/User/Login", self.base_url))
            .send_form(&login_data)
            .await?;

        let cookies = response.cookies().context("Unable to parse a cookie")?;
        let auth_cookie = cookies.iter().find(|c| c.name() == ".AspNetCore.Cookies");

        match auth_cookie {
            None => Err(VocadbClientError::BadCredentialsError),
            Some(cookie) => {
                self.clear_cookie();
                self.add_cookie(cookie);
                Ok(())
            }
        }
    }

    pub async fn current_user(&self) -> Result<UserForApiContract> {
        self.http_get(
            &format!("{}/api/users/current", self.base_url),
            &vec![("fields", OptionalFields::MainPicture.as_ref().to_string())],
        )
        .await
    }

    pub async fn get_messages(
        &self,
        user_id: i32,
        start_offset: i32,
        max_results: i32,
    ) -> Result<PartialFindResult<UserMessageContract>> {
        debug!("Get messages for user: {}", user_id);

        let query = vec![
            ("inbox", Inbox::Notifications.as_ref().to_string()),
            ("start", start_offset.to_string()),
            ("maxResults", max_results.to_string()),
            ("getTotalCount", true.to_string()),
        ];

        self.http_get(
            &format!("{}/api/users/{}/messages", self.base_url, user_id),
            &query,
        )
        .await
    }

    pub async fn get_message(&self, message_id: i32) -> Result<UserMessageContract> {
        debug!("Get message by ID: {}", message_id);

        let url: String = format!("{}/api/users/messages/{}", self.base_url, message_id);
        let query: Vec<(&str, &str)> = vec![];
        self.http_get(&url, &query).await
    }

    pub async fn delete_messages(&self, user_id: i32, message_ids: &[i32]) -> Result<()> {
        debug!("Delete messages by IDs: {:?}", message_ids);

        let query: Vec<(&str, String)> = message_ids
            .iter()
            .map(|id| ("messageId", id.to_string()))
            .collect();
        self.http_delete_void(
            &format!("{}/api/users/{}/messages", self.base_url, user_id),
            &query,
        )
        .await
    }

    pub async fn get_song_by_id(
        &self,
        song_id: i32,
        language: &LanguagePreference,
    ) -> Result<SongForApiContract> {
        debug!("Get song by ID: {}", song_id);

        let default_fields = vec![
            OptionalFields::AdditionalNames.as_ref(),
            OptionalFields::Names.as_ref(),
            OptionalFields::MainPicture.as_ref(),
            OptionalFields::PVs.as_ref(),
            OptionalFields::Tags.as_ref(),
            OptionalFields::ThumbUrl.as_ref(),
            OptionalFields::WebLinks.as_ref(),
        ]
        .join(",");

        self.http_get(
            &format!("{}/api/songs/{}", self.base_url, song_id),
            &vec![
                ("fields", default_fields.as_str()),
                ("lang", language.as_ref()),
            ],
        )
        .await
    }
}
