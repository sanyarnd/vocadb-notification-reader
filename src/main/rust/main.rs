use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

mod client;
mod controller;
mod errors;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/api/login", web::post().to(controller::login))
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(service::auth::validate_token))
                    .route(
                        "/notifications/fetch",
                        web::post().to(controller::get_notifications),
                    )
                    .route(
                        "/notifications/delete",
                        web::post().to(controller::delete_notifications),
                    )
                    .route(
                        "/users/current",
                        web::get().to(controller::get_current_user),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
