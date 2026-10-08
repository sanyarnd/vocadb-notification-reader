//! Thin HTTP client for the VocaDB family of websites.

pub mod models;

use std::sync::Mutex;

use chrono::{DateTime, TimeDelta, Utc};
use reqwest::header::{COOKIE, HeaderMap, LOCATION, SET_COOKIE};
use reqwest::{RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, info};

use crate::client::models::{
    Inbox, LanguagePreference, OptionalFields, PartialFindResult, UserForApiContract,
    UserMessageContract,
};

pub type Result<T, E = ClientError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Provided credentials are not valid")]
    BadCredentials,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("Unexpected response: {0}")]
    InvalidResponse(#[from] serde_json::Error),
}

const AUTH_COOKIE: &str = ".AspNetCore.Cookies";

/// Client bound to a single VocaDB-like website and a user session.
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    jar: Mutex<Jar>,
}

/// VocaDB auth cookies. ASP.NET may split a large auth cookie into chunks
/// (`.AspNetCore.CookiesC1`, ...), so every auth related cookie is kept.
#[derive(Default, Debug)]
struct Jar {
    cookies: Vec<(String, String)>,
    expires_at: Option<DateTime<Utc>>,
    changed: bool,
}

impl Jar {
    fn header(&self) -> Option<String> {
        (!self.cookies.is_empty()).then(|| {
            self.cookies
                .iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join("; ")
        })
    }

    fn is_authenticated(&self) -> bool {
        self.cookies.iter().any(|(name, _)| name == AUTH_COOKIE)
    }

    /// Applies `Set-Cookie` headers: VocaDB may refresh or remove its cookies on any response.
    fn absorb(&mut self, headers: &HeaderMap, now: DateTime<Utc>) {
        for header in headers.get_all(SET_COOKIE) {
            let Some(cookie) = header
                .to_str()
                .ok()
                .and_then(|h| cookie::Cookie::parse(h).ok())
            else {
                continue;
            };
            if !cookie.name().starts_with(AUTH_COOKIE) {
                continue;
            }

            let expires_at = cookie
                .max_age()
                .map(|max_age| now + TimeDelta::seconds(max_age.whole_seconds()))
                .or_else(|| {
                    cookie
                        .expires_datetime()
                        .and_then(|e| DateTime::from_timestamp(e.unix_timestamp(), 0))
                });
            let removed = cookie.value().is_empty() || expires_at.is_some_and(|e| e <= now);

            self.cookies.retain(|(name, _)| name != cookie.name());
            if !removed {
                self.cookies
                    .push((cookie.name().to_string(), cookie.value().to_string()));
            }
            if cookie.name() == AUTH_COOKIE {
                self.expires_at = expires_at;
            }
            self.changed = true;
        }
    }
}

/// VocaDB rejects an ended session either with an error status or by redirecting to the
/// login page.
fn is_signed_out(response: &reqwest::Response) -> bool {
    let status = response.status();
    let redirects_to_login = status.is_redirection()
        && response
            .headers()
            .get(LOCATION)
            .and_then(|location| location.to_str().ok())
            .is_some_and(|location| location.to_ascii_lowercase().contains("/user/login"));

    status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN || redirects_to_login
}

impl Client {
    /// Creates a client for a session; `cookies` are in the `name=value` form.
    pub fn new(
        http: reqwest::Client,
        base_url: impl Into<String>,
        cookies: &[String],
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let cookies = cookies
            .iter()
            .filter_map(|pair| pair.split_once('='))
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();

        Client {
            http,
            base_url: base_url.into(),
            jar: Mutex::new(Jar {
                cookies,
                expires_at,
                changed: false,
            }),
        }
    }

    /// Session cookies in the `name=value` form.
    pub fn cookies(&self) -> Vec<String> {
        let jar = self.jar.lock().unwrap();
        jar.cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect()
    }

    /// Expiration of the auth cookie, if VocaDB announced one.
    pub fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.jar.lock().unwrap().expires_at
    }

    /// Whether VocaDB changed the cookies since the last call.
    pub fn take_cookies_changed(&self) -> bool {
        std::mem::take(&mut self.jar.lock().unwrap().changed)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn send(&self, builder: RequestBuilder) -> Result<reqwest::Response> {
        let cookie_header = self.jar.lock().unwrap().header();
        let builder = match cookie_header {
            Some(header) => builder.header(COOKIE, header),
            None => builder,
        };

        let response = builder.send().await?;
        self.jar
            .lock()
            .unwrap()
            .absorb(response.headers(), Utc::now());

        if is_signed_out(&response) {
            return Err(ClientError::BadCredentials);
        }
        Ok(response.error_for_status()?)
    }

    /// Performs a GET request and returns the raw JSON body.
    async fn get_text<Q>(&self, path: &str, query: &Q) -> Result<String>
    where
        Q: Serialize + ?Sized,
    {
        let url = self.url(path);
        debug!("GET {url}");
        let response = self.send(self.http.get(url).query(query)).await?;
        Ok(response.text().await?)
    }

    async fn get<Q, R>(&self, path: &str, query: &Q) -> Result<R>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        Ok(serde_json::from_str(&self.get_text(path, query).await?)?)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
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

        *self.jar.lock().unwrap() = Jar::default();
        self.send(self.http.post(self.url("/User/Login")).form(&LoginForm {
            username,
            password,
            keep_logged_in: true,
        }))
        .await?;

        if self.jar.lock().unwrap().is_authenticated() {
            Ok(())
        } else {
            Err(ClientError::BadCredentials)
        }
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

    /// Raw JSON of a message, see [`UserMessageContract`].
    pub async fn get_message_json(&self, message_id: i32) -> Result<String> {
        self.get_text(
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

    /// Raw JSON of a song, see [`models::SongForApiContract`].
    pub async fn get_song_json(
        &self,
        song_id: i32,
        language: LanguagePreference,
    ) -> Result<String> {
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

        self.get_text(
            &format!("/api/songs/{song_id}"),
            &[("fields", fields.as_str()), ("lang", language.as_ref())],
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;

    use super::*;

    fn headers(values: &[&str]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for value in values {
            headers.append(SET_COOKIE, HeaderValue::from_str(value).unwrap());
        }
        headers
    }

    #[test]
    fn keeps_auth_cookies_with_expiration() {
        let now = Utc::now();
        let mut jar = Jar::default();
        jar.absorb(
            &headers(&[
                "unrelated=1; path=/",
                ".AspNetCore.Cookies=chunks-2; expires=Fri, 01 Jan 2100 00:00:00 GMT; path=/",
                ".AspNetCore.CookiesC1=part1; path=/",
                ".AspNetCore.CookiesC2=part2; path=/",
            ]),
            now,
        );

        assert!(jar.is_authenticated());
        assert!(jar.changed);
        assert_eq!(
            jar.header().unwrap(),
            ".AspNetCore.Cookies=chunks-2; .AspNetCore.CookiesC1=part1; .AspNetCore.CookiesC2=part2"
        );
        assert_eq!(
            jar.expires_at,
            DateTime::parse_from_rfc3339("2100-01-01T00:00:00Z")
                .map(|d| d.to_utc())
                .ok()
        );
    }

    #[test]
    fn refreshes_and_removes_cookies() {
        let now = Utc::now();
        let mut jar = Jar::default();
        jar.absorb(
            &headers(&[".AspNetCore.Cookies=old", ".AspNetCore.CookiesC1=part"]),
            now,
        );
        assert_eq!(jar.expires_at, None);

        jar.absorb(
            &headers(&[
                ".AspNetCore.Cookies=new; max-age=3600",
                ".AspNetCore.CookiesC1=; expires=Thu, 01 Jan 1970 00:00:00 GMT",
            ]),
            now,
        );
        assert_eq!(jar.header().unwrap(), ".AspNetCore.Cookies=new");
        assert_eq!(jar.expires_at, Some(now + TimeDelta::hours(1)));

        // Signing out removes the auth cookie.
        jar.absorb(&headers(&[".AspNetCore.Cookies=; max-age=0"]), now);
        assert!(!jar.is_authenticated());
        assert_eq!(jar.header(), None);
    }

    #[test]
    fn unrelated_cookies_are_not_a_change() {
        let mut jar = Jar::default();
        jar.absorb(&headers(&["unrelated=1", "garbage"]), Utc::now());
        assert!(!jar.changed);
        assert_eq!(jar.header(), None);
    }

    #[test]
    fn restores_cookies_from_session() {
        let client = Client::new(
            reqwest::Client::new(),
            "https://vocadb.net",
            &[".AspNetCore.Cookies=abc=def".to_string()],
            None,
        );
        assert_eq!(client.cookies(), vec![".AspNetCore.Cookies=abc=def"]);
        assert!(!client.take_cookies_changed());
    }
}
