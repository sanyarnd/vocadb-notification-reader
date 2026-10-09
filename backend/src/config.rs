use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Context;
use ipnet::IpNet;

use crate::service::Database;

/// Application settings, read from environment variables.
pub struct Config {
    /// `LISTEN_ADDR`, defaults to `0.0.0.0:8080`.
    pub listen_addr: SocketAddr,
    /// `REDIS_URL`: Valkey/Redis used to store sessions, e.g. `redis://valkey:6379/0`.
    /// Sessions are kept in memory when it is missing.
    pub redis_url: Option<String>,
    /// `CORS_ALLOWED_ORIGINS`: comma separated list of origins allowed to call the API.
    pub cors_allowed_origins: Vec<String>,
    /// `TRUSTED_PROXIES`: comma separated networks of reverse proxies whose
    /// `X-Forwarded-For` is trusted, defaults to loopback.
    pub trusted_proxies: Vec<IpNet>,
    /// Base URL for every supported database (overridable for testing).
    pub database_urls: HashMap<Database, String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let listen_addr = std::env::var("LISTEN_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
            .parse()
            .context("LISTEN_ADDR must be a socket address")?;

        let redis_url = std::env::var("REDIS_URL")
            .ok()
            .filter(|url| !url.is_empty());

        let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .map(|origins| parse_list(&origins))
            .unwrap_or_default();

        let trusted_proxies = crate::client_ip::parse_networks(
            &std::env::var("TRUSTED_PROXIES")
                .unwrap_or_else(|_| DEFAULT_TRUSTED_PROXIES.to_string()),
        )
        .context("Invalid TRUSTED_PROXIES")?;

        Ok(Config {
            listen_addr,
            redis_url,
            cors_allowed_origins,
            trusted_proxies,
            database_urls: default_database_urls(),
        })
    }
}

pub const DEFAULT_TRUSTED_PROXIES: &str = "127.0.0.1/32,::1/128";

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
