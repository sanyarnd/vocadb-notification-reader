use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::middleware::Logger;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use awc::http::StatusCode;

use crate::web::controller;
use crate::web::errors::{collect_stacktrace, ErrorResponse};

mod client;
mod service;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .app_data(actix_web::web::JsonConfig::default().error_handler(json_error_handler))
            .wrap(Logger::default())
            .service(controller::login)
            .service(
                actix_web::web::scope("/api")
                    .wrap(HttpAuthentication::bearer(
                        web::middleware::auth_token::validate,
                    ))
                    .service(controller::get_notifications)
                    .service(controller::delete_notifications)
                    .service(controller::get_current_user),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let stacktrace = collect_stacktrace(&err);
    let message = err.to_string();
    let code = StatusCode::BAD_REQUEST.as_u16();
    let response = HttpResponse::BadRequest().json(&ErrorResponse {
        code,
        message,
        stacktrace,
    });
    InternalError::from_response(err, response).into()
}
