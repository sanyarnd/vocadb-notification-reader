//! Fixed window rate limits stored in Valkey/Redis, so they hold across replicas.
//!
//! Limits fail open: if the storage is unavailable requests are let through.

use std::time::Duration;

use sha2::{Digest, Sha256};
use tracing::warn;

use crate::kv::{KEY_PREFIX, Kv};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit {
    pub name: &'static str,
    pub requests: u64,
    pub window: Duration,
}

const fn minutes(n: u64) -> Duration {
    Duration::from_secs(n * 60)
}

/// Every request to the API from one address. Generous, as many users may share
/// an address behind CGNAT.
pub const PER_IP: Limit = Limit {
    name: "ip",
    requests: 600,
    window: minutes(1),
};
/// Login attempts from one address.
pub const LOGIN_PER_IP: Limit = Limit {
    name: "login-ip",
    requests: 20,
    window: minutes(15),
};
/// Login attempts to one account, from any address.
pub const LOGIN_PER_ACCOUNT: Limit = Limit {
    name: "login-account",
    requests: 10,
    window: minutes(15),
};
/// Requests within one session.
pub const PER_SESSION: Limit = Limit {
    name: "session",
    requests: 120,
    window: minutes(1),
};

#[derive(Clone)]
pub struct RateLimiter {
    kv: Kv,
}

impl RateLimiter {
    pub fn new(kv: Kv) -> Self {
        RateLimiter { kv }
    }

    /// Counts a request against `limit` for `subject`; returns the time to wait
    /// when the limit is exceeded.
    pub async fn check(&self, limit: Limit, subject: &str) -> Option<Duration> {
        // Subjects may be personal (usernames, session IDs), keys only contain their hash.
        let key = format!(
            "{KEY_PREFIX}rate:{}:{}",
            limit.name,
            hex::encode(Sha256::digest(subject.as_bytes()))
        );
        match self.kv.increment(&key, limit.window).await {
            Ok((count, reset)) => (count > limit.requests).then_some(reset),
            Err(e) => {
                warn!("Rate limit check failed: {e:#}");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: Limit = Limit {
        name: "test",
        requests: 3,
        window: minutes(1),
    };

    #[tokio::test]
    async fn limits_per_subject() {
        let limiter = RateLimiter::new(Kv::memory());

        for _ in 0..3 {
            assert_eq!(limiter.check(LIMIT, "a").await, None);
        }
        let retry_after = limiter.check(LIMIT, "a").await.expect("limited");
        assert!(retry_after <= LIMIT.window);

        assert_eq!(limiter.check(LIMIT, "b").await, None);
    }

    #[tokio::test(start_paused = true)]
    async fn window_resets() {
        let limiter = RateLimiter::new(Kv::memory());
        for _ in 0..4 {
            limiter.check(LIMIT, "a").await;
        }
        assert!(limiter.check(LIMIT, "a").await.is_some());

        tokio::time::advance(LIMIT.window + Duration::from_secs(1)).await;
        assert_eq!(limiter.check(LIMIT, "a").await, None);
    }
}
