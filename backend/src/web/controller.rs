use actix_web::{post, Responder, Result};
use actix_web::http::header::Header;
use actix_web::HttpRequest;
use actix_web::web::Json;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use anyhow::Context;

use crate::client::http_client::Client;
use crate::client::errors::VocadbClientError;
use crate::service::notifications::{Database, load_notification_details};
use crate::web::dto::{
    LoginRequest, LoginResponse, NotificationsDeleteRequest, NotificationsFetchRequest,
    NotificationsFetchResponse, Token,
};
use crate::web::errors::AppResponseError;
use crate::web::middleware::auth_token;

#[post("/users/current")]
pub async fn get_current_user(req: HttpRequest) -> Result<impl Responder, AppResponseError> {
    let token = extract_token(&req)?;
    let client = client_from_token(&token)?;

    let user = client.current_user().await?;
    Ok(Json(user))
}

#[post("/api/login")]
pub async fn login(payload: Json<LoginRequest>) -> Result<impl Responder, AppResponseError> {
    let mut client = create_client(&payload.database, &[])?;
    client.login(&payload.username, &payload.password).await?;

    let user = client.current_user().await?;

    let token = auth_token::encode(user.id, &payload.database, &client.cookies)?;

    Ok(Json(LoginResponse { token }))
}

#[post("/notifications/fetch")]
pub async fn get_notifications(
    req: HttpRequest,
    payload: Json<NotificationsFetchRequest>,
) -> Result<impl Responder, AppResponseError> {
    if payload.start_offset < 0 || payload.max_results < 0 || payload.max_results > 100 {
        return Err(AppResponseError::ConstraintViolation(
            "Invalid parameters".to_string(),
        ));
    }

    let token = extract_token(&req)?;
    let client = client_from_token(&token)?;

    let notifications = client
        .get_messages(token.user_id, payload.start_offset, payload.max_results)
        .await?;

    let futures = notifications
        .items
        .iter()
        .map(|pm| load_notification_details(&client, &token.database, &payload.language, pm.id));
    let messages = futures::future::try_join_all(futures).await?;

    Ok(Json(NotificationsFetchResponse {
        notifications: messages,
        total_count: notifications.total_count,
    }))
}

#[post("/notifications/delete")]
pub async fn delete_notifications(
    req: HttpRequest,
    payload: Json<NotificationsDeleteRequest>,
) -> Result<impl Responder, AppResponseError> {
    let token = extract_token(&req)?;
    let client = client_from_token(&token)?;

    client.delete_messages(token.user_id, &payload.ids).await?;

    Ok(Json(()))
}

fn extract_token(req: &HttpRequest) -> Result<Token, AppResponseError> {
    let auth = Authorization::<Bearer>::parse(req).context("Unable to parse Bearer header")?;
    let token = auth_token::parse(auth.as_ref().token())?;

    Ok(token)
}

fn client_from_token(token: &Token) -> Result<Client, AppResponseError> {
    Ok(create_client(&token.database, &token.cookies)?)
}

fn create_client<'a>(
    database: &Database,
    cookies: &'a [String],
) -> Result<Client<'a>, VocadbClientError> {
    match database {
        Database::VocaDb => Client::vocadb(cookies),
        Database::TouhouDb => Client::touhoudb(cookies),
        Database::UtaiteDb => Client::utaitedb(cookies),
    }
}
