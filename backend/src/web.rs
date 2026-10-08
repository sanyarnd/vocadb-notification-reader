//! HTTP API consumed by the frontend.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use axum::extract::{FromRequest, FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, header};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::client::Client;
use crate::client::models::{LanguagePreference, UserForApiContract};
use crate::config::Config;
use crate::error::AppError;
use crate::service::dto::Notification;
use crate::service::{Database, load_notification_details};
use crate::token::{Token, TokenCodec};

pub type Result<T, E = AppError> = core::result::Result<T, E>;

/// Maximum amount of notifications requested at once.
pub const MAX_RESULTS: i32 = 100;
/// Maximum amount of simultaneous requests to VocaDB per API call.
const UPSTREAM_CONCURRENCY: usize = 8;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    token_codec: TokenCodec,
    database_urls: HashMap<Database, String>,
}

impl AppState {
    pub fn new(
        token_codec: TokenCodec,
        database_urls: HashMap<Database, String>,
    ) -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .context("Unable to create an HTTP client")?;

        Ok(AppState {
            inner: Arc::new(Inner {
                http,
                token_codec,
                database_urls,
            }),
        })
    }

    fn client(&self, database: Database, cookies: Vec<String>) -> Result<Client> {
        let base_url = self
            .inner
            .database_urls
            .get(&database)
            .with_context(|| format!("No URL configured for {database:?}"))?;
        Ok(Client::new(
            self.inner.http.clone(),
            base_url.clone(),
            cookies,
        ))
    }
}

pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/login", post(login))
        .route("/users/current", post(current_user))
        .route("/notifications/fetch", post(fetch_notifications))
        .route("/notifications/delete", post(delete_notifications));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", api)
        .with_state(state)
}

pub fn app(config: Config) -> anyhow::Result<Router> {
    let state = AppState::new(config.token_codec, config.database_urls)?;
    let mut app = router(state).layer(TraceLayer::new_for_http());

    if !config.cors_allowed_origins.is_empty() {
        let origins = config
            .cors_allowed_origins
            .iter()
            .map(|origin| HeaderValue::from_str(origin))
            .collect::<Result<Vec<_>, _>>()
            .context("Invalid CORS origin")?;
        app = app.layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods([Method::POST])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                // The API may sit behind an authenticating proxy relying on cookies.
                .allow_credentials(true),
        );
    }

    Ok(app)
}

/// JSON extractor that reports malformed payloads with [`AppError`].
#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

/// Session restored from the bearer token.
pub struct Session(pub Token);

impl FromRequestParts<AppState> for Session {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let token = state
            .inner
            .token_codec
            .decode(bearer.token(), Utc::now())
            .map_err(|e| AppError::Unauthorized(format!("{e:#}")))?;

        Ok(Session(token))
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsFetchRequest {
    pub start_offset: i32,
    pub max_results: i32,
    pub language: LanguagePreference,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsFetchResponse {
    pub notifications: Vec<Notification>,
    pub total_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct NotificationsDeleteRequest {
    pub ids: Vec<i32>,
}

async fn login(
    State(state): State<AppState>,
    AppJson(payload): AppJson<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let mut client = state.client(payload.database, vec![])?;
    client.login(&payload.username, &payload.password).await?;
    let user = client.current_user().await?;

    let token = Token::new(
        user.id,
        payload.database,
        client.cookies().to_vec(),
        Utc::now(),
    );
    let token = state.inner.token_codec.encode(&token)?;

    Ok(Json(LoginResponse { token }))
}

async fn current_user(
    State(state): State<AppState>,
    Session(token): Session,
) -> Result<Json<UserForApiContract>> {
    let client = state.client(token.database, token.cookies)?;
    Ok(Json(client.current_user().await?))
}

async fn fetch_notifications(
    State(state): State<AppState>,
    Session(token): Session,
    AppJson(payload): AppJson<NotificationsFetchRequest>,
) -> Result<Json<NotificationsFetchResponse>> {
    if payload.start_offset < 0 || !(0..=MAX_RESULTS).contains(&payload.max_results) {
        return Err(AppError::ConstraintViolation(format!(
            "startOffset must be non-negative and maxResults must be within [0, {MAX_RESULTS}]"
        )));
    }

    let client = state.client(token.database, token.cookies)?;
    let messages = client
        .get_messages(token.user_id, payload.start_offset, payload.max_results)
        .await?;

    let notifications = futures::stream::iter(messages.items)
        .map(|message| {
            load_notification_details(&client, token.database, payload.language, message.id)
        })
        .buffered(UPSTREAM_CONCURRENCY)
        .try_collect()
        .await?;

    Ok(Json(NotificationsFetchResponse {
        notifications,
        total_count: messages.total_count,
    }))
}

async fn delete_notifications(
    State(state): State<AppState>,
    Session(token): Session,
    AppJson(payload): AppJson<NotificationsDeleteRequest>,
) -> Result<Json<()>> {
    if !payload.ids.is_empty() {
        let client = state.client(token.database, token.cookies)?;
        client.delete_messages(token.user_id, &payload.ids).await?;
    }
    Ok(Json(()))
}
