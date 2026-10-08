//! HTTP API consumed by the frontend.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use axum::extract::{FromRequest, FromRequestParts, Query, Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use ts_rs::TS;

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
pub const MAX_LIMIT: i32 = 100;
/// Maximum amount of simultaneous requests to VocaDB per API call.
const UPSTREAM_CONCURRENCY: usize = 8;

/// Session cookie. The `__Host-` prefix pins it to the API host over HTTPS.
pub const SESSION_COOKIE: &str = "__Host-session";
/// Browsers cap cookie lifetime at 400 days; the cookie is re-issued on every `GET /api/me`.
const SESSION_COOKIE_MAX_AGE: Duration = Duration::from_secs(400 * 24 * 60 * 60);

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    sessions: SessionStore,
    cache: Cache,
    database_urls: HashMap<Database, String>,
    allowed_origins: Vec<String>,
}

impl AppState {
    pub fn new(
        kv: Kv,
        database_urls: HashMap<Database, String>,
        allowed_origins: Vec<String>,
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
                sessions: SessionStore::new(kv.clone()),
                cache: Cache::new(kv),
                database_urls,
                allowed_origins,
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
        .route("/session", axum::routing::post(login).delete(logout))
        .route("/me", get(me))
        .route(
            "/notifications",
            get(fetch_notifications).delete(delete_notifications),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), check_origin));

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
    let state = AppState::new(
        kv,
        config.database_urls,
        config.cors_allowed_origins.clone(),
    )?;
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
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers([header::CONTENT_TYPE])
                .allow_credentials(true),
        );
    }

    Ok(app)
}

/// CSRF protection: state changing requests from browsers must come from the API's own
/// origin or one of the allowed origins. `SameSite` alone doesn't cover sibling subdomains.
async fn check_origin(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if matches!(
        *request.method(),
        Method::GET | Method::HEAD | Method::OPTIONS
    ) {
        return next.run(request).await;
    }

    let headers = request.headers();
    let Some(origin) = headers.get(header::ORIGIN) else {
        // Browsers always send Origin with such requests; other clients aren't a CSRF vector.
        return next.run(request).await;
    };
    let origin = origin.to_str().unwrap_or_default();
    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    let same_origin = origin
        .split_once("://")
        .is_some_and(|(_, authority)| !host.is_empty() && authority == host);
    let allowed = state.inner.allowed_origins.iter().any(|o| o == origin);

    if same_origin || allowed {
        next.run(request).await
    } else {
        AppError::Forbidden(format!("Requests from {origin} are not allowed")).into_response()
    }
}

/// JSON extractor that reports malformed payloads with [`AppError`].
#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

/// Query extractor that reports malformed parameters with [`AppError`].
#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(AppError))]
pub struct AppQuery<T>(pub T);

/// Session referenced by the session cookie.
pub struct Authenticated {
    pub id: String,
    pub session: Session,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let jar = CookieJar::from_headers(&parts.headers);
        let id = jar
            .get(SESSION_COOKIE)
            .map(|c| c.value().to_string())
            .ok_or_else(|| AppError::Unauthorized("Not signed in".to_string()))?;
        let session = state
            .inner
            .sessions
            .get(&id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session has ended".to_string()))?;

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

fn session_cookie(id: String) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, id))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(
            SESSION_COOKIE_MAX_AGE
                .try_into()
                .expect("max age must fit into cookie duration"),
        )
        .build()
}

fn removed_session_cookie() -> Cookie<'static> {
    let mut cookie = session_cookie(String::new());
    cookie.make_removal();
    cookie
}

#[derive(Deserialize, TS, Debug)]
#[ts(export)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub database: Database,
}

#[derive(Serialize, TS, Debug)]
#[ts(export)]
pub struct Account {
    pub database: Database,
    pub user: UserForApiContract,
}

#[derive(Deserialize, TS, Debug)]
#[ts(export)]
pub struct NotificationsQuery {
    pub offset: i32,
    pub limit: i32,
    pub language: LanguagePreference,
}

#[derive(Serialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NotificationsResponse {
    pub notifications: Vec<Notification>,
    pub total_count: i32,
}

#[derive(Deserialize, TS, Debug)]
#[ts(export)]
pub struct DeleteNotificationsRequest {
    pub ids: Vec<i32>,
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    AppJson(payload): AppJson<LoginRequest>,
) -> Result<(CookieJar, Json<Account>)> {
    let client = state.client(payload.database, None)?;
    client.login(&payload.username, &payload.password).await?;
    let user = client.current_user().await?;

    let session = Session {
        user_id: user.id,
        database: payload.database,
        cookies: client.cookies(),
        expires_at: client.expires_at(),
    };
    let id = state.inner.sessions.create(&session).await?;

    Ok((
        jar.add(session_cookie(id)),
        Json(Account {
            database: payload.database,
            user,
        }),
    ))
}

async fn logout(State(state): State<AppState>, jar: CookieJar) -> Result<(CookieJar, Json<()>)> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        state.inner.sessions.delete(cookie.value()).await?;
    }
    Ok((jar.add(removed_session_cookie()), Json(())))
}

async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
    auth: Authenticated,
) -> Result<(CookieJar, Json<Account>)> {
    let client = auth.client(&state)?;
    let result = client.current_user().await.map_err(AppError::from);
    let user = auth.sync(&state, &client, result).await?;

    Ok((
        jar.add(session_cookie(auth.id)),
        Json(Account {
            database: auth.session.database,
            user,
        }),
    ))
}

async fn fetch_notifications(
    State(state): State<AppState>,
    auth: Authenticated,
    AppQuery(query): AppQuery<NotificationsQuery>,
) -> Result<Json<NotificationsResponse>> {
    if query.offset < 0 || !(0..=MAX_LIMIT).contains(&query.limit) {
        return Err(AppError::ConstraintViolation(format!(
            "offset must be non-negative and limit must be within [0, {MAX_LIMIT}]"
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
            .get_messages(source.user_id, query.offset, query.limit)
            .await?;

        let notifications = futures::stream::iter(messages.items)
            .map(|message| load_notification_details(&source, query.language, message.id))
            .buffered(UPSTREAM_CONCURRENCY)
            .try_collect()
            .await?;

        Ok(NotificationsResponse {
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
    AppJson(payload): AppJson<DeleteNotificationsRequest>,
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
