use serde::{Deserialize, Serialize};

use crate::client::models::query::LanguagePreference;
use crate::service::dto::Notification;
use crate::service::notifications::Database;

#[derive(Deserialize, Debug)]
pub struct NotificationsFetchRequest {
    #[serde(rename = "startOffset")]
    pub start_offset: i32,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    pub language: LanguagePreference,
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

#[derive(Serialize)]
pub struct NotificationsFetchResponse {
    pub notifications: Vec<Box<dyn Notification>>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub user_id: i32,
    pub database: Database,
    pub cookies: Vec<String>,
    pub exp: i64,
}
