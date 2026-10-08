use anyhow::Context;
use tracing_subscriber::EnvFilter;
use vocadb_notification_reader::config::Config;
use vocadb_notification_reader::web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;
    let listener = tokio::net::TcpListener::bind(config.listen_addr)
        .await
        .with_context(|| format!("Unable to bind {}", config.listen_addr))?;
    tracing::info!("Listening on {}", config.listen_addr);

    axum::serve(listener, web::app(config)?)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server failed")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.ok();
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut signal) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
