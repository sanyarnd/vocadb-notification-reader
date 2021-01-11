use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::time::Duration;

use actix_web::client::ClientRequest;
use actix_web::cookie::Cookie;
use actix_web::http::Method;
use actix_web::HttpMessage;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::errors::ClientError;
use crate::client::models::misc::PartialFindResult;
use crate::client::models::query::{LanguagePreference, OptionalFields};
use crate::client::models::song::SongForApiContract;
use crate::client::models::user::{Inbox, UserForApiContract, UserMessageContract};

pub type ClientCookies<'a> = Option<Vec<Cookie<'a>>>;
pub type Result<T, E = ClientError> = core::result::Result<T, E>;

pub struct Client<'a> {
    pub base_url: &'a str,
    pub cookies: RefCell<ClientCookies<'a>>,
}

impl From<&'static str> for Client<'_> {
    fn from(value: &'static str) -> Self {
        return Client {
            base_url: value,
            cookies: RefCell::new(None),
        };
    }
}

impl<'a> Client<'a> {
    pub fn new(base_url: &'a str, cookies: Vec<String>) -> Result<Client<'a>> {
        let mut cookies_ = vec![];
        for c in cookies {
            let cookie = Cookie::parse(c)?;
            cookies_.push(cookie)
        }

        return Ok(Client {
            base_url,
            cookies: RefCell::new(Some(cookies_)),
        });
    }

    fn base_url(&self) -> &'a str {
        return self.base_url;
    }

    fn set_cookies(&self, cookies: Ref<Vec<Cookie<'a>>>) -> ClientCookies<'a> {
        return self.cookies.replace(Some(cookies.to_owned()));
    }

    pub fn default_client(&self) -> actix_web::client::Client {
        return actix_web::client::Client::builder()
            .connector(actix_web::client::Connector::new()
                .timeout(Duration::from_secs(30))
                .finish())
            .timeout(Duration::from_secs(30))
            .finish();
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        let client = self.default_client();

        #[derive(Serialize)]
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

        let response = client
            .post(format!("{}/User/Login", self.base_url()))
            .send_form(&login_data)
            .await?;
        let cookies = response.cookies()?;

        let result = cookies.borrow().len() != 0;
        return if result {
            self.set_cookies(cookies);
            Ok(())
        } else {
            Err(ClientError::BadCredentialsError)
        };
    }

    fn create_request(&self, url: &String, method: actix_web::http::Method) -> ClientRequest {
        let client = self.default_client();
        let mut builder = match method {
            Method::GET => client.get(url),
            Method::POST => client.post(url),
            Method::DELETE => client.delete(url),
            _ => panic!("Unsupported method"),
        };

        let cookies_ref = self.cookies.borrow();
        if let Some(cookies) = cookies_ref.deref() {
            for c in cookies {
                builder = builder.cookie(c.clone());
            }
        }
        return builder;
    }

    async fn http_get<T>(&self, url: &String, query: &Vec<(&str, String)>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let request = self.create_request(url, Method::GET);
        let body = request.query(query)?.send().await?.body().await?;
        let json = serde_json::from_slice(&body)?;
        return Ok(json);
    }

    async fn http_delete<T>(&self, url: &String, query: &Vec<(&str, String)>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let request = self.create_request(url, Method::DELETE);
        let body = request.query(query)?.send().await?.body().await?;
        let json = serde_json::from_slice(&body)?;
        return Ok(json);
    }

    pub async fn current_user(&self) -> Result<UserForApiContract> {
        self.http_get(
            &format!("{}/api/users/current", self.base_url),
            &vec![("fields", OptionalFields::MainPicture.to_string())],
        )
        .await
    }

    pub async fn get_messages(
        &self,
        user_id: i32,
        start_offset: i32,
        max_results: i32,
    ) -> Result<PartialFindResult<UserMessageContract>> {
        log::info!("Get messages for user: {}", user_id);

        self.http_get(
            &format!("{}/api/users/{}/messages", self.base_url, user_id),
            &vec![
                ("inbox", Inbox::Notifications.to_string()),
                ("start", start_offset.to_string()),
                ("maxResults", max_results.to_string()),
                ("getTotalCount", true.to_string()),
            ],
        )
        .await
    }

    pub async fn get_message(&self, message_id: i32) -> Result<UserMessageContract> {
        log::info!("Get message by ID: {}", message_id);

        self.http_get(
            &format!("{}/api/users/messages/{}", self.base_url, message_id),
            &vec![],
        )
        .await
    }

    pub async fn delete_messages(&self, user_id: i32, message_ids: &Vec<i32>) -> Result<()> {
        log::info!("Delete messages by IDs: {:?}", message_ids);

        self.http_delete(
            &format!("{}/api/users/{}/messages", self.base_url, user_id),
            &message_ids
                .iter()
                .map(|id| ("messageId", id.to_string()))
                .collect(),
        )
        .await
    }

    pub async fn get_song_by_id(
        &self,
        song_id: i32,
        language: &LanguagePreference,
    ) -> Result<SongForApiContract> {
        log::info!("Get song by ID: {}", song_id);

        self.http_get(
            &format!("{}/api/songs/{}", self.base_url, song_id),
            &vec![
                (
                    "fields",
                    OptionalFields::AdditionalNames.to_string()
                        + ","
                        + &OptionalFields::Names.to_string()
                        + ","
                        + &OptionalFields::MainPicture.to_string()
                        + ","
                        + &OptionalFields::PVs.to_string()
                        + ","
                        + &OptionalFields::Tags.to_string()
                        + ","
                        + &OptionalFields::ThumbUrl.to_string()
                        + ","
                        + &OptionalFields::WebLinks.to_string(),
                ),
                ("lang", language.to_string()),
            ],
        )
        .await
    }
}
