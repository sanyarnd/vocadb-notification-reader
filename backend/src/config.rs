use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Context;

use crate::service::Database;
use crate::token::TokenCodec;

/// Application settings, read from environment variables.
pub struct Config {
    /// `LISTEN_ADDR`, defaults to `0.0.0.0:8080`.
    pub listen_addr: SocketAddr,
    /// `TOKEN_KEY`: hex encoded 32-byte key. A random key is used when it is missing.
    pub token_codec: TokenCodec,
    /// `CORS_ALLOWED_ORIGINS`: comma separated list of origins allowed to call the API.
    pub cors_allowed_origins: Vec<String>,
    /// Base URL for every supported database (overridable for testing).
    pub database_urls: HashMap<Database, String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let listen_addr = std::env::var("LISTEN_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
            .parse()
            .context("LISTEN_ADDR must be a socket address")?;

        let token_codec = match std::env::var("TOKEN_KEY") {
            Ok(key) => TokenCodec::from_hex(&key).context("Invalid TOKEN_KEY")?,
            Err(_) => {
                tracing::warn!("TOKEN_KEY is not set, sessions will not survive a restart");
                TokenCodec::random()
            }
        };

        let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .map(|origins| parse_list(&origins))
            .unwrap_or_default();

        Ok(Config {
            listen_addr,
            token_codec,
            cors_allowed_origins,
            database_urls: default_database_urls(),
        })
    }
}

pub fn default_database_urls() -> HashMap<Database, String> {
    Database::ALL
        .into_iter()
        .map(|db| (db, db.default_url()))
        .collect()
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comma_separated_list() {
        assert_eq!(
            parse_list(" https://a.example , ,https://b.example"),
            vec!["https://a.example", "https://b.example"]
        );
        assert!(parse_list("").is_empty());
    }
}
