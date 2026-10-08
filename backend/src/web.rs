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
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::cache::Cache;
use crate::client::models::{LanguagePreference, UserForApiContract};
use crate::client::{Client, ClientError};
use crate::config::Config;
use crate::error::AppError;
use crate::kv::Kv;
use crate::service::dto::Notification;
use crate::service::{Database, Source, load_notification_details};
use crate::session::{Session, SessionStore};

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
    sessions: SessionStore,
    cache: Cache,
    database_urls: HashMap<Database, String>,
}

impl AppState {
    pub fn new(kv: Kv, database_urls: HashMap<Database, String>) -> anyhow::Result<Self> {
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
                sessions: SessionStore::new(kv.clone()),
                cache: Cache::new(kv),
                database_urls,
            }),
        })
    }

    fn client(&self, database: Database, session: Option<&Session>) -> Result<Client> {
        let base_url = self
            .inner
            .database_urls
            .get(&database)
            .with_context(|| format!("No URL configured for {database:?}"))?;
        Ok(Client::new(
            self.inner.http.clone(),
            base_url.clone(),
            session.map_or(&[], |s| s.cookies.as_slice()),
            session.and_then(|s| s.expires_at),
        ))
    }
}

pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/users/current", post(current_user))
        .route("/notifications/fetch", post(fetch_notifications))
        .route("/notifications/delete", post(delete_notifications));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", api)
        .with_state(state)
}

pub async fn app(config: Config) -> anyhow::Result<Router> {
    let kv = match &config.redis_url {
        Some(url) => Kv::redis(url).await?,
        None => {
            tracing::warn!("REDIS_URL is not set, sessions and cache are kept in memory");
            Kv::memory()
        }
    };
    let state = AppState::new(kv, config.database_urls)?;
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

/// Session referenced by the bearer token.
pub struct Authenticated {
    pub id: String,
    pub session: Session,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let id = bearer.token().to_string();
        let session = state
            .inner
            .sessions
            .get(&id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session has expired".to_string()))?;

        Ok(Authenticated { id, session })
    }
}

impl Authenticated {
    fn client(&self, state: &AppState) -> Result<Client> {
        state.client(self.session.database, Some(&self.session))
    }

    /// Keeps the stored session in sync with VocaDB: saves refreshed cookies and
    /// forgets the session once VocaDB rejects it.
    async fn sync<T>(&self, state: &AppState, client: &Client, result: Result<T>) -> Result<T> {
        let sessions = &state.inner.sessions;
        if matches!(result, Err(AppError::Client(ClientError::BadCredentials))) {
            tracing::info!("VocaDB session of user {} has ended", self.session.user_id);
            if let Err(e) = sessions.delete(&self.id).await {
                tracing::warn!("{e:#}");
            }
        } else if client.take_cookies_changed() {
            let session = Session {
                cookies: client.cookies(),
                expires_at: client.expires_at(),
                ..self.session.clone()
            };
            if let Err(e) = sessions.update(&self.id, &session).await {
                tracing::warn!("{e:#}");
            }
        }
        result
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
    let client = state.client(payload.database, None)?;
    client.login(&payload.username, &payload.password).await?;
    let user = client.current_user().await?;

    let session = Session {
        user_id: user.id,
        database: payload.database,
        cookies: client.cookies(),
        expires_at: client.expires_at(),
    };
    let token = state.inner.sessions.create(&session).await?;

    Ok(Json(LoginResponse { token }))
}

async fn logout(State(state): State<AppState>, auth: Authenticated) -> Result<Json<()>> {
    state.inner.sessions.delete(&auth.id).await?;
    Ok(Json(()))
}

async fn current_user(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<UserForApiContract>> {
    let client = auth.client(&state)?;
    let result = client.current_user().await.map_err(AppError::from);
    Ok(Json(auth.sync(&state, &client, result).await?))
}

async fn fetch_notifications(
    State(state): State<AppState>,
    auth: Authenticated,
    AppJson(payload): AppJson<NotificationsFetchRequest>,
) -> Result<Json<NotificationsFetchResponse>> {
    if payload.start_offset < 0 || !(0..=MAX_RESULTS).contains(&payload.max_results) {
        return Err(AppError::ConstraintViolation(format!(
            "startOffset must be non-negative and maxResults must be within [0, {MAX_RESULTS}]"
        )));
    }

    let client = auth.client(&state)?;
    let source = Source {
        client: &client,
        cache: &state.inner.cache,
        database: auth.session.database,
        user_id: auth.session.user_id,
    };
    let result = async {
        let messages = client
            .get_messages(source.user_id, payload.start_offset, payload.max_results)
            .await?;

        let notifications = futures::stream::iter(messages.items)
            .map(|message| load_notification_details(&source, payload.language, message.id))
            .buffered(UPSTREAM_CONCURRENCY)
            .try_collect()
            .await?;

        Ok(NotificationsFetchResponse {
            notifications,
            total_count: messages.total_count,
        })
    }
    .await;

    Ok(Json(auth.sync(&state, &client, result).await?))
}

async fn delete_notifications(
    State(state): State<AppState>,
    auth: Authenticated,
    AppJson(payload): AppJson<NotificationsDeleteRequest>,
) -> Result<Json<()>> {
    if payload.ids.is_empty() {
        return Ok(Json(()));
    }

    let client = auth.client(&state)?;
    let Session {
        database, user_id, ..
    } = auth.session;
    let result = client
        .delete_messages(user_id, &payload.ids)
        .await
        .map_err(AppError::from);
    auth.sync(&state, &client, result).await?;

    let keys: Vec<String> = payload
        .ids
        .iter()
        .map(|&id| Cache::message_key(database, user_id, id))
        .collect();
    state.inner.cache.evict(&keys).await;

    Ok(Json(()))
}
