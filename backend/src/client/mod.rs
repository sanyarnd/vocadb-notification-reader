//! Thin HTTP client for the VocaDB family of websites.

pub mod models;

use reqwest::header::{COOKIE, SET_COOKIE};
use reqwest::{RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, info};

use crate::client::models::{
    Inbox, LanguagePreference, OptionalFields, PartialFindResult, SongForApiContract,
    UserForApiContract, UserMessageContract,
};

pub type Result<T, E = ClientError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Provided credentials are not valid")]
    BadCredentials,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

const AUTH_COOKIE: &str = ".AspNetCore.Cookies";

/// Client bound to a single VocaDB-like website and a user session.
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    cookies: Vec<String>,
}

impl Client {
    pub fn new(http: reqwest::Client, base_url: impl Into<String>, cookies: Vec<String>) -> Self {
        Client {
            http,
            base_url: base_url.into(),
            cookies,
        }
    }

    /// Session cookies in the `name=value` form.
    pub fn cookies(&self) -> &[String] {
        &self.cookies
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn with_cookies(&self, builder: RequestBuilder) -> RequestBuilder {
        if self.cookies.is_empty() {
            builder
        } else {
            builder.header(COOKIE, self.cookies.join("; "))
        }
    }

    async fn send(&self, builder: RequestBuilder) -> Result<reqwest::Response> {
        let response = self.with_cookies(builder).send().await?;
        match response.status() {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(ClientError::BadCredentials),
            _ => Ok(response.error_for_status()?),
        }
    }

    async fn get<Q, R>(&self, path: &str, query: &Q) -> Result<R>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.url(path);
        debug!("GET {url}");
        let response = self.send(self.http.get(url).query(query)).await?;
        Ok(response.json().await?)
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        #[derive(Serialize)]
        struct LoginForm<'a> {
            #[serde(rename = "UserName")]
            username: &'a str,
            #[serde(rename = "Password")]
            password: &'a str,
            #[serde(rename = "KeepLoggedIn")]
            keep_logged_in: bool,
        }

        info!("Logging in user {username}");

        let response = self
            .http
            .post(self.url("/User/Login"))
            .form(&LoginForm {
                username,
                password,
                keep_logged_in: true,
            })
            .send()
            .await?;

        // ASP.NET may split a large auth cookie into chunks (`.AspNetCore.CookiesC1`, ...),
        // so every auth related cookie is kept.
        let cookies: Vec<String> = response
            .headers()
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .filter_map(|value| value.split(';').next())
            .map(str::trim)
            .filter(|pair| pair.starts_with(AUTH_COOKIE))
            .map(String::from)
            .collect();

        let authenticated = cookies.iter().any(|pair| {
            pair.split_once('=')
                .is_some_and(|(name, _)| name == AUTH_COOKIE)
        });
        if !authenticated {
            return Err(ClientError::BadCredentials);
        }

        self.cookies = cookies;
        Ok(())
    }

    pub async fn current_user(&self) -> Result<UserForApiContract> {
        self.get(
            "/api/users/current",
            &[("fields", OptionalFields::MainPicture.as_ref())],
        )
        .await
    }

    pub async fn get_messages(
        &self,
        user_id: i32,
        start_offset: i32,
        max_results: i32,
    ) -> Result<PartialFindResult<UserMessageContract>> {
        debug!("Get messages for user {user_id}");

        self.get(
            &format!("/api/users/{user_id}/messages"),
            &[
                ("inbox", Inbox::Notifications.as_ref().to_string()),
                ("start", start_offset.to_string()),
                ("maxResults", max_results.to_string()),
                ("getTotalCount", true.to_string()),
            ],
        )
        .await
    }

    pub async fn get_message(&self, message_id: i32) -> Result<UserMessageContract> {
        self.get(
            &format!("/api/users/messages/{message_id}"),
            &[] as &[(&str, &str)],
        )
        .await
    }

    pub async fn delete_messages(&self, user_id: i32, message_ids: &[i32]) -> Result<()> {
        debug!("Delete messages {message_ids:?}");

        let query: Vec<(&str, String)> = message_ids
            .iter()
            .map(|id| ("messageId", id.to_string()))
            .collect();
        self.send(
            self.http
                .delete(self.url(&format!("/api/users/{user_id}/messages")))
                .query(&query),
        )
        .await?;

        Ok(())
    }

    pub async fn get_song_by_id(
        &self,
        song_id: i32,
        language: LanguagePreference,
    ) -> Result<SongForApiContract> {
        let fields = [
            OptionalFields::AdditionalNames,
            OptionalFields::Names,
            OptionalFields::MainPicture,
            OptionalFields::PVs,
            OptionalFields::Tags,
            OptionalFields::ThumbUrl,
            OptionalFields::WebLinks,
        ]
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<&str>>()
        .join(",");

        self.get(
            &format!("/api/songs/{song_id}"),
            &[("fields", fields.as_str()), ("lang", language.as_ref())],
        )
        .await
    }
}
