use std::error::Error;

use axum::Json;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use ts_rs::TS;

use crate::client::ClientError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("Invalid request payload: {0}")]
    InvalidPayload(#[from] JsonRejection),
    #[error("Invalid query: {0}")]
    InvalidQuery(#[from] QueryRejection),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Too many requests, retry in {} s", .0.as_secs())]
    TooManyRequests(std::time::Duration),
    #[error("Web client error: {0}")]
    Client(#[from] ClientError),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Serialize, TS, Debug)]
#[ts(export)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub stacktrace: Vec<String>,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ConstraintViolation(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidPayload(rejection) => rejection.status(),
            AppError::InvalidQuery(rejection) => rejection.status(),
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::Unauthorized(_) | AppError::Client(ClientError::BadCredentials) => {
                StatusCode::UNAUTHORIZED
            }
            AppError::Client(ClientError::Http(_) | ClientError::InvalidResponse(_)) => {
                StatusCode::BAD_GATEWAY
            }
            AppError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = self.status_code();
        if code.is_server_error() {
            tracing::error!("{self:#}");
        }

        let body = ErrorResponse {
            code: code.as_u16(),
            message: self.to_string(),
            stacktrace: collect_stacktrace(&self),
        };
        let mut response = (code, Json(body)).into_response();
        if let AppError::TooManyRequests(retry_after) = self {
            // Round up, a client retrying early would hit the limit again.
            let seconds = retry_after.as_secs() + u64::from(retry_after.subsec_nanos() > 0);
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(seconds.max(1)));
        }
        response
    }
}

pub fn collect_stacktrace(err: &dyn Error) -> Vec<String> {
    std::iter::successors(err.source(), |e| (*e).source())
        .map(ToString::to_string)
        .collect()
}
