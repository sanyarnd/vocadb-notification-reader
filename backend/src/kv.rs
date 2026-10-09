//! Minimal key-value storage used for sessions and caching.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Context;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use tokio::time::Instant;

/// Prefix for every key written by the application.
pub const KEY_PREFIX: &str = "vocadb-notification-reader:";

#[derive(Clone)]
pub enum Kv {
    Redis(ConnectionManager),
    /// Process local storage, intended for development and tests.
    Memory(Arc<Mutex<HashMap<String, (String, Instant)>>>),
}

impl Kv {
    pub async fn redis(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url).context("Invalid REDIS_URL")?;
        let manager = client
            .get_connection_manager()
            .await
            .context("Unable to connect to Valkey/Redis")?;
        Ok(Kv::Redis(manager))
    }

    pub fn memory() -> Self {
        Kv::Memory(Arc::default())
    }

    pub async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        match self {
            Kv::Redis(redis) => Ok(redis.clone().get(key).await?),
            Kv::Memory(map) => {
                let mut map = map.lock().unwrap();
                match map.get(key) {
                    Some((value, expires)) if *expires > Instant::now() => Ok(Some(value.clone())),
                    Some(_) => {
                        map.remove(key);
                        Ok(None)
                    }
                    None => Ok(None),
                }
            }
        }
    }

    pub async fn set(&self, key: &str, value: &str, ttl: Duration) -> anyhow::Result<()> {
        let ttl = ttl.max(Duration::from_secs(1));
        match self {
            Kv::Redis(redis) => {
                let _: () = redis.clone().set_ex(key, value, ttl.as_secs()).await?;
            }
            Kv::Memory(map) => {
                let mut map = map.lock().unwrap();
                let now = Instant::now();
                map.retain(|_, (_, expires)| *expires > now);
                map.insert(key.to_string(), (value.to_string(), now + ttl));
            }
        }
        Ok(())
    }

    pub async fn expire(&self, key: &str, ttl: Duration) -> anyhow::Result<()> {
        let ttl = ttl.max(Duration::from_secs(1));
        match self {
            Kv::Redis(redis) => {
                let _: () = redis.clone().expire(key, ttl.as_secs() as i64).await?;
            }
            Kv::Memory(map) => {
                if let Some((_, expires)) = map.lock().unwrap().get_mut(key) {
                    *expires = Instant::now() + ttl;
                }
            }
        }
        Ok(())
    }

    pub async fn delete(&self, keys: &[String]) -> anyhow::Result<()> {
        if keys.is_empty() {
            return Ok(());
        }
        match self {
            Kv::Redis(redis) => {
                let _: () = redis.clone().del(keys).await?;
            }
            Kv::Memory(map) => {
                let mut map = map.lock().unwrap();
                for key in keys {
                    map.remove(key);
                }
            }
        }
        Ok(())
    }

    /// Increments a counter that expires `window` after its first increment.
    /// Returns the new value and the time left until the counter resets.
    pub async fn increment(&self, key: &str, window: Duration) -> anyhow::Result<(u64, Duration)> {
        let window = window.max(Duration::from_secs(1));
        match self {
            Kv::Redis(redis) => {
                let (count, (), ttl): (u64, (), i64) = redis::pipe()
                    .atomic()
                    .incr(key, 1)
                    .cmd("EXPIRE")
                    .arg(key)
                    .arg(window.as_secs())
                    .arg("NX")
                    .ttl(key)
                    .query_async(&mut redis.clone())
                    .await?;
                let ttl = if ttl > 0 {
                    Duration::from_secs(ttl as u64)
                } else {
                    window
                };
                Ok((count, ttl))
            }
            Kv::Memory(map) => {
                let mut map = map.lock().unwrap();
                let now = Instant::now();
                let entry = map
                    .entry(key.to_string())
                    .or_insert_with(|| ("0".to_string(), now + window));
                if entry.1 <= now {
                    *entry = ("0".to_string(), now + window);
                }
                let count = entry.0.parse::<u64>().unwrap_or(0) + 1;
                entry.0 = count.to_string();
                Ok((count, entry.1 - now))
            }
        }
    }

    /// Remaining lifetime of a key, `None` when it doesn't exist.
    pub async fn ttl(&self, key: &str) -> anyhow::Result<Option<Duration>> {
        match self {
            Kv::Redis(redis) => {
                let ttl: i64 = redis.clone().ttl(key).await?;
                Ok((ttl >= 0).then(|| Duration::from_secs(ttl as u64)))
            }
            Kv::Memory(map) => {
                let now = Instant::now();
                Ok(map
                    .lock()
                    .unwrap()
                    .get(key)
                    .filter(|(_, expires)| *expires > now)
                    .map(|(_, expires)| *expires - now))
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Connects to a real Valkey/Redis when `TEST_REDIS_URL` is set (it is in CI).
    pub(crate) async fn redis() -> Option<Kv> {
        match std::env::var("TEST_REDIS_URL") {
            Ok(url) => Some(Kv::redis(&url).await.unwrap()),
            Err(_) => {
                eprintln!("TEST_REDIS_URL is not set, skipping");
                None
            }
        }
    }

    async fn check(kv: Kv) {
        let key = format!("{KEY_PREFIX}test:{}", std::process::id());
        let hour = Duration::from_secs(3600);

        assert_eq!(kv.get(&key).await.unwrap(), None);
        assert_eq!(kv.ttl(&key).await.unwrap(), None);

        kv.set(&key, "value", hour).await.unwrap();
        assert_eq!(kv.get(&key).await.unwrap().as_deref(), Some("value"));
        let ttl = kv.ttl(&key).await.unwrap().unwrap();
        assert!(
            ttl <= hour && ttl > hour - Duration::from_secs(5),
            "{ttl:?}"
        );

        kv.expire(&key, 2 * hour).await.unwrap();
        assert!(kv.ttl(&key).await.unwrap().unwrap() > hour);

        kv.delete(std::slice::from_ref(&key)).await.unwrap();
        assert_eq!(kv.get(&key).await.unwrap(), None);
        kv.delete(&[]).await.unwrap();

        let counter = format!("{key}:counter");
        for expected in 1..=3 {
            let (count, ttl) = kv.increment(&counter, hour).await.unwrap();
            assert_eq!(count, expected);
            assert!(
                ttl <= hour && ttl > hour - Duration::from_secs(5),
                "{ttl:?}"
            );
        }
        kv.delete(&[counter]).await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn memory_counters_reset_after_window() {
        let kv = Kv::memory();
        let window = Duration::from_secs(60);
        assert_eq!(kv.increment("c", window).await.unwrap().0, 1);
        tokio::time::advance(Duration::from_secs(30)).await;
        let (count, ttl) = kv.increment("c", window).await.unwrap();
        assert_eq!((count, ttl), (2, Duration::from_secs(30)));
        tokio::time::advance(Duration::from_secs(31)).await;
        assert_eq!(kv.increment("c", window).await.unwrap().0, 1);
    }

    #[tokio::test]
    async fn memory() {
        check(Kv::memory()).await;
    }

    #[tokio::test]
    async fn redis_backend() {
        if let Some(kv) = redis().await {
            check(kv).await;
        }
    }

    #[tokio::test(start_paused = true)]
    async fn memory_entries_expire() {
        let kv = Kv::memory();
        kv.set("key", "value", Duration::from_secs(10))
            .await
            .unwrap();

        tokio::time::advance(Duration::from_secs(9)).await;
        assert!(kv.get("key").await.unwrap().is_some());
        tokio::time::advance(Duration::from_secs(2)).await;
        assert!(kv.get("key").await.unwrap().is_none());
    }
}
