//! Best-effort cache for VocaDB responses.
//!
//! Failures of the underlying storage never fail a request, they only cost an upstream call.

use std::time::Duration;

use serde::de::DeserializeOwned;
use tracing::warn;

use crate::client::models::LanguagePreference;
use crate::client::{ClientError, Result};
use crate::kv::{KEY_PREFIX, Kv};
use crate::service::Database;

/// Messages never change once sent.
pub const MESSAGE_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);
/// The list of messages changes as notifications arrive, it's only kept between page loads.
pub const INBOX_TTL: Duration = Duration::from_secs(60);
/// Songs are edited from time to time (tags, PVs).
pub const SONG_TTL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone)]
pub struct Cache {
    kv: Kv,
}

impl Cache {
    pub fn new(kv: Kv) -> Self {
        Cache { kv }
    }

    /// Messages are private, so the key includes their recipient.
    pub fn message_key(database: Database, user_id: i32, message_id: i32) -> String {
        format!("{KEY_PREFIX}cache:message:{database:?}:{user_id}:{message_id}")
    }

    pub fn inbox_key(database: Database, user_id: i32) -> String {
        format!("{KEY_PREFIX}cache:inbox:{database:?}:{user_id}")
    }

    pub fn song_key(database: Database, song_id: i32, language: LanguagePreference) -> String {
        format!(
            "{KEY_PREFIX}cache:song:{database:?}:{song_id}:{}",
            language.as_ref()
        )
    }

    /// Returns a cached JSON document or fetches it. Only documents that can be
    /// deserialized are cached.
    pub async fn get_or_fetch<T, F>(&self, key: &str, ttl: Duration, fetch: F) -> Result<T>
    where
        T: DeserializeOwned,
        F: Future<Output = Result<String>>,
    {
        match self.kv.get(key).await {
            Ok(Some(json)) => match serde_json::from_str(&json) {
                Ok(value) => return Ok(value),
                Err(e) => warn!("Ignoring a broken cache entry {key}: {e}"),
            },
            Ok(None) => {}
            Err(e) => warn!("Cache lookup failed: {e:#}"),
        }

        let json = fetch.await?;
        let value = serde_json::from_str(&json).map_err(ClientError::from)?;
        if let Err(e) = self.kv.set(key, &json, ttl).await {
            warn!("Cache update failed: {e:#}");
        }
        Ok(value)
    }

    pub async fn evict(&self, keys: &[String]) {
        if let Err(e) = self.kv.delete(keys).await {
            warn!("Cache eviction failed: {e:#}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[tokio::test]
    async fn fetches_once() {
        let cache = Cache::new(Kv::memory());
        let calls = AtomicUsize::new(0);
        let fetch = || async {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok("[1, 2]".to_string())
        };

        for _ in 0..3 {
            let value: Vec<i32> = cache.get_or_fetch("key", SONG_TTL, fetch()).await.unwrap();
            assert_eq!(value, [1, 2]);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        cache.evict(&["key".to_string()]).await;
        let _: Vec<i32> = cache.get_or_fetch("key", SONG_TTL, fetch()).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn does_not_cache_failures_and_garbage() {
        let cache = Cache::new(Kv::memory());

        let result: Result<Vec<i32>> = cache
            .get_or_fetch("key", SONG_TTL, async { Err(ClientError::BadCredentials) })
            .await;
        assert!(matches!(result, Err(ClientError::BadCredentials)));

        let result: Result<Vec<i32>> = cache
            .get_or_fetch("key", SONG_TTL, async { Ok("<html>".to_string()) })
            .await;
        assert!(matches!(result, Err(ClientError::InvalidResponse(_))));

        let value: Vec<i32> = cache
            .get_or_fetch("key", SONG_TTL, async { Ok("[3]".to_string()) })
            .await
            .unwrap();
        assert_eq!(value, [3]);
    }

    #[tokio::test]
    async fn refetches_broken_entries() {
        let kv = Kv::memory();
        kv.set("key", "{broken", SONG_TTL).await.unwrap();

        let value: Vec<i32> = Cache::new(kv.clone())
            .get_or_fetch("key", SONG_TTL, async { Ok("[4]".to_string()) })
            .await
            .unwrap();
        assert_eq!(value, [4]);
        assert_eq!(kv.get("key").await.unwrap().as_deref(), Some("[4]"));
    }

    #[test]
    fn keys_are_scoped() {
        assert_ne!(
            Cache::message_key(Database::VocaDb, 1, 10),
            Cache::message_key(Database::VocaDb, 2, 10)
        );
        assert_ne!(
            Cache::message_key(Database::VocaDb, 1, 10),
            Cache::message_key(Database::UtaiteDb, 1, 10)
        );
        assert_ne!(
            Cache::song_key(Database::VocaDb, 1, LanguagePreference::Romaji),
            Cache::song_key(Database::VocaDb, 1, LanguagePreference::English)
        );
    }
}
