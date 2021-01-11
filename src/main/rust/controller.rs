use std::convert::TryFrom;

use actix_web::http::header::Header;
use actix_web::web;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures::future;
use serde::Deserialize;
use serde::Serialize;

use crate::client::client::Client;
use crate::client::models::query::LanguagePreference;
use crate::client::models::user::UserForApiContract;
use crate::errors::AppError;
use crate::service::auth::create_token;
use crate::service::auth::Claims;
use crate::service::auth::Database;
use crate::service::dto::Notification;
use crate::service::notifications::load_notification_details;

#[derive(Deserialize, Debug)]
pub struct NotificationsFetchRequest {
    #[serde(rename = "startOffset")]
    pub start_offset: i32,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    pub language: LanguagePreference,
}

#[derive(Serialize)]
pub struct NotificationsFetchResponse {
    pub notifications: Vec<Box<dyn Notification>>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct NotificationsDeleteRequest {
    pub ids: Vec<i32>,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub database: Database,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn get_current_user(req: HttpRequest) -> Result<Json<UserForApiContract>, AppError> {
    let client = Client::try_from(&req)?;

    let user = client.current_user().await?;
    return Ok(Json(user));
}

pub async fn login(payload: web::Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    let token = create_token(&payload.username, &payload.password, &payload.database).await?;
    return Ok(Json(LoginResponse { token }));
}

pub async fn get_notifications(
    req: HttpRequest,
    payload: Json<NotificationsFetchRequest>,
) -> Result<Json<NotificationsFetchResponse>, AppError> {
    if payload.start_offset < 0 || payload.max_results < 0 || payload.max_results > 100 {
        return Err(AppError::ConstraintViolationError);
    }

    let client = Client::try_from(&req)?;
    let claims = Claims::try_from(&req)?;

    let notifications = client
        .get_messages(claims.user_id, payload.start_offset, payload.max_results)
        .await?;

    let futures = notifications
        .items
        .iter()
        .map(|pm| load_notification_details(&client, &claims.database, &payload.language, pm.id));
    let messages = future::try_join_all(futures).await?;

    return Ok(Json(NotificationsFetchResponse {
        notifications: messages,
        total_count: notifications.total_count,
    }));
}

pub async fn delete_notifications(
    req: HttpRequest,
    payload: Json<NotificationsDeleteRequest>,
) -> Result<Json<()>, AppError> {
    let client = Client::try_from(&req)?;
    let claims = Claims::try_from(&req)?;

    client.delete_messages(claims.user_id, &payload.ids).await?;
    return Ok(Json(()));
}

impl TryFrom<&HttpRequest> for Client<'_> {
    type Error = AppError;

    fn try_from(req: &HttpRequest) -> Result<Self, Self::Error> {
        let claims = Claims::try_from(req)?;

        let client = Client::new(claims.database.url(), claims.cookies.clone())?;
        return Ok(client);
    }
}

impl TryFrom<&HttpRequest> for Claims {
    type Error = AppError;

    fn try_from(req: &HttpRequest) -> Result<Self, Self::Error> {
        let authorization = Authorization::<Bearer>::parse(req)?;
        let token = crate::service::auth::decode_token(authorization.as_ref().token())?;
        return Ok(token.claims);
    }
}
