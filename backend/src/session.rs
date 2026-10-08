//! Server side sessions.
//!
//! The client only gets an opaque random session ID; VocaDB cookies stay on the server.
//! Sessions are stored in Valkey/Redis under a SHA-256 of the ID, so a storage dump
//! doesn't contain usable bearer tokens.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Context;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::time::Instant;

use crate::service::Database;

/// Sessions expire after this period of inactivity.
pub const SESSION_TTL: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const KEY_PREFIX: &str = "vocadb-notification-reader:session:";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user_id: i32,
    pub database: Database,
    pub cookies: Vec<String>,
}

#[derive(Clone)]
pub enum SessionStore {
    Redis(ConnectionManager),
    /// Process local storage, intended for development and tests.
    Memory(Arc<Mutex<HashMap<String, (Session, Instant)>>>),
}

impl SessionStore {
    pub async fn redis(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url).context("Invalid session store URL")?;
        let manager = client
            .get_connection_manager()
            .await
            .context("Unable to connect to the session store")?;
        Ok(SessionStore::Redis(manager))
    }

    pub fn memory() -> Self {
        SessionStore::Memory(Arc::default())
    }

    /// Stores a session and returns its ID.
    pub async fn create(&self, session: &Session) -> anyhow::Result<String> {
        let id = generate_id()?;
        let key = storage_key(&id);
        match self {
            SessionStore::Redis(redis) => {
                let value =
                    serde_json::to_string(session).context("Unable to serialize a session")?;
                let _: () = redis
                    .clone()
                    .set_ex(key, value, SESSION_TTL.as_secs())
                    .await
                    .context("Unable to store a session")?;
            }
            SessionStore::Memory(map) => {
                let mut map = map.lock().unwrap();
                let now = Instant::now();
                map.retain(|_, (_, expires)| *expires > now);
                map.insert(key, (session.clone(), now + SESSION_TTL));
            }
        }
        Ok(id)
    }

    /// Looks a session up and extends its lifetime.
    pub async fn get(&self, id: &str) -> anyhow::Result<Option<Session>> {
        let key = storage_key(id);
        match self {
            SessionStore::Redis(redis) => {
                let value: Option<String> = redis
                    .clone()
                    .get_ex(key, redis::Expiry::EX(SESSION_TTL.as_secs()))
                    .await
                    .context("Unable to load a session")?;
                value
                    .map(|v| serde_json::from_str(&v).context("Unable to deserialize a session"))
                    .transpose()
            }
            SessionStore::Memory(map) => {
                let mut map = map.lock().unwrap();
                let now = Instant::now();
                match map.get_mut(&key) {
                    Some((session, expires)) if *expires > now => {
                        *expires = now + SESSION_TTL;
                        Ok(Some(session.clone()))
                    }
                    Some(_) => {
                        map.remove(&key);
                        Ok(None)
                    }
                    None => Ok(None),
                }
            }
        }
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let key = storage_key(id);
        match self {
            SessionStore::Redis(redis) => {
                let _: () = redis
                    .clone()
                    .del(key)
                    .await
                    .context("Unable to delete a session")?;
            }
            SessionStore::Memory(map) => {
                map.lock().unwrap().remove(&key);
            }
        }
        Ok(())
    }
}

fn generate_id() -> anyhow::Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|e| anyhow::anyhow!("Unable to generate a session ID: {e}"))?;
    Ok(hex::encode(bytes))
}

fn storage_key(id: &str) -> String {
    format!("{KEY_PREFIX}{}", hex::encode(Sha256::digest(id.as_bytes())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> Session {
        Session {
            user_id: 42,
            database: Database::TouhouDb,
            cookies: vec![".AspNetCore.Cookies=abc".to_string()],
        }
    }

    /// Exercises the store contract; shared by the in-memory and Redis tests.
    async fn check_store(store: SessionStore) {
        let id = store.create(&session()).await.unwrap();
        assert_eq!(id.len(), 64);
        assert_eq!(store.get(&id).await.unwrap(), Some(session()));

        let other = store.create(&session()).await.unwrap();
        assert_ne!(id, other);

        store.delete(&id).await.unwrap();
        assert_eq!(store.get(&id).await.unwrap(), None);
        assert_eq!(store.get(&other).await.unwrap(), Some(session()));
        assert_eq!(store.get("unknown").await.unwrap(), None);
    }

    #[tokio::test]
    async fn memory_store() {
        check_store(SessionStore::memory()).await;
    }

    #[tokio::test(start_paused = true)]
    async fn memory_store_expires_idle_sessions() {
        let store = SessionStore::memory();
        let id = store.create(&session()).await.unwrap();

        // Every access extends the session.
        tokio::time::advance(SESSION_TTL - Duration::from_secs(1)).await;
        assert!(store.get(&id).await.unwrap().is_some());
        tokio::time::advance(SESSION_TTL - Duration::from_secs(1)).await;
        assert!(store.get(&id).await.unwrap().is_some());

        tokio::time::advance(SESSION_TTL + Duration::from_secs(1)).await;
        assert!(store.get(&id).await.unwrap().is_none());
    }

    /// Runs against a real Valkey/Redis when `TEST_REDIS_URL` is set (it is in CI).
    #[tokio::test]
    async fn redis_store() {
        let Ok(url) = std::env::var("TEST_REDIS_URL") else {
            eprintln!("TEST_REDIS_URL is not set, skipping");
            return;
        };
        let store = SessionStore::redis(&url).await.unwrap();
        check_store(store.clone()).await;

        // The raw ID must not be stored, and the TTL must be set.
        let id = store.create(&session()).await.unwrap();
        let SessionStore::Redis(mut redis) = store else {
            unreachable!()
        };
        let raw: Option<String> = redis.get(format!("{KEY_PREFIX}{id}")).await.unwrap();
        assert_eq!(raw, None);
        let ttl: i64 = redis.ttl(storage_key(&id)).await.unwrap();
        assert!(ttl > 0 && ttl <= SESSION_TTL.as_secs() as i64, "{ttl}");
    }

    #[test]
    fn storage_key_hides_the_id() {
        let key = storage_key("secret");
        assert!(key.starts_with(KEY_PREFIX));
        assert!(!key.contains("secret"));
        assert_eq!(key, storage_key("secret"));
    }
}
