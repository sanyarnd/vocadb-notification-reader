//! Server side sessions.
//!
//! The client only gets an opaque random session ID; VocaDB cookies stay on the server.
//! Sessions are stored under a SHA-256 of the ID, so a storage dump doesn't contain
//! usable bearer tokens.
//!
//! A session lives as long as VocaDB keeps the user signed in: its lifetime follows
//! the expiration of the VocaDB auth cookie, and it is removed as soon as VocaDB
//! rejects the cookies.

use std::time::Duration;

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::kv::{KEY_PREFIX, Kv};
use crate::service::Database;

/// Lifetime of sessions whose VocaDB cookie has no expiration date, extended on every use.
/// Only abandoned sessions ever reach it.
pub const IDLE_TTL: Duration = Duration::from_secs(365 * 24 * 60 * 60);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user_id: i32,
    pub database: Database,
    /// VocaDB cookies in the `name=value` form.
    pub cookies: Vec<String>,
    /// Expiration of the VocaDB auth cookie, as announced by VocaDB.
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
}

impl Session {
    fn ttl(&self, now: DateTime<Utc>) -> Duration {
        match self.expires_at {
            Some(expires_at) => (expires_at - now).to_std().unwrap_or_default(),
            None => IDLE_TTL,
        }
    }
}

#[derive(Clone)]
pub struct SessionStore {
    kv: Kv,
}

impl SessionStore {
    pub fn new(kv: Kv) -> Self {
        SessionStore { kv }
    }

    /// Stores a new session and returns its ID.
    pub async fn create(&self, session: &Session) -> anyhow::Result<String> {
        let id = generate_id()?;
        self.update(&id, session).await?;
        Ok(id)
    }

    /// Replaces the session data, e.g. after VocaDB refreshed its cookies.
    pub async fn update(&self, id: &str, session: &Session) -> anyhow::Result<()> {
        let value = serde_json::to_string(session).context("Unable to serialize a session")?;
        self.kv
            .set(&storage_key(id), &value, session.ttl(Utc::now()))
            .await
            .context("Unable to store a session")
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<Session>> {
        let key = storage_key(id);
        let Some(value) = self
            .kv
            .get(&key)
            .await
            .context("Unable to load a session")?
        else {
            return Ok(None);
        };
        let session: Session =
            serde_json::from_str(&value).context("Unable to deserialize a session")?;

        if session.expires_at.is_none() {
            self.kv
                .expire(&key, IDLE_TTL)
                .await
                .context("Unable to extend a session")?;
        }
        Ok(Some(session))
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.kv
            .delete(&[storage_key(id)])
            .await
            .context("Unable to delete a session")
    }
}

fn generate_id() -> anyhow::Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|e| anyhow::anyhow!("Unable to generate a session ID: {e}"))?;
    Ok(hex::encode(bytes))
}

fn storage_key(id: &str) -> String {
    format!(
        "{KEY_PREFIX}session:{}",
        hex::encode(Sha256::digest(id.as_bytes()))
    )
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    fn session(expires_at: Option<DateTime<Utc>>) -> Session {
        Session {
            user_id: 42,
            database: Database::TouhouDb,
            cookies: vec![".AspNetCore.Cookies=abc".to_string()],
            expires_at,
        }
    }

    async fn check_store(kv: Kv) {
        let store = SessionStore::new(kv.clone());

        let id = store.create(&session(None)).await.unwrap();
        assert_eq!(id.len(), 64);
        assert_eq!(store.get(&id).await.unwrap(), Some(session(None)));
        assert!(kv.ttl(&storage_key(&id)).await.unwrap().unwrap() > IDLE_TTL / 2);

        let other = store.create(&session(None)).await.unwrap();
        assert_ne!(id, other);

        // The lifetime follows the VocaDB cookie.
        let expires_at = Utc::now() + TimeDelta::days(30);
        store.update(&id, &session(Some(expires_at))).await.unwrap();
        assert_eq!(
            store.get(&id).await.unwrap(),
            Some(session(Some(expires_at)))
        );
        let ttl = kv.ttl(&storage_key(&id)).await.unwrap().unwrap();
        assert!(ttl <= Duration::from_secs(30 * 24 * 3600), "{ttl:?}");
        assert!(ttl > Duration::from_secs(29 * 24 * 3600), "{ttl:?}");

        store.delete(&id).await.unwrap();
        assert_eq!(store.get(&id).await.unwrap(), None);
        assert_eq!(store.get(&other).await.unwrap(), Some(session(None)));
        assert_eq!(store.get("unknown").await.unwrap(), None);

        // The raw ID is never used as a key.
        assert_eq!(
            kv.get(&format!("{KEY_PREFIX}session:{other}"))
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn memory_store() {
        check_store(Kv::memory()).await;
    }

    #[tokio::test]
    async fn redis_store() {
        if let Some(kv) = crate::kv::tests::redis().await {
            check_store(kv).await;
        }
    }

    #[tokio::test(start_paused = true)]
    async fn sessions_without_cookie_expiration_are_extended_on_use() {
        let store = SessionStore::new(Kv::memory());
        let id = store.create(&session(None)).await.unwrap();

        for _ in 0..3 {
            tokio::time::advance(IDLE_TTL - Duration::from_secs(1)).await;
            assert!(store.get(&id).await.unwrap().is_some());
        }

        tokio::time::advance(IDLE_TTL + Duration::from_secs(1)).await;
        assert!(store.get(&id).await.unwrap().is_none());
    }

    #[test]
    fn expired_cookie_means_expired_session() {
        let now = Utc::now();
        assert_eq!(
            session(Some(now - TimeDelta::days(1))).ttl(now),
            Duration::ZERO
        );
        assert_eq!(session(None).ttl(now), IDLE_TTL);
    }

    #[test]
    fn storage_key_hides_the_id() {
        let key = storage_key("secret");
        assert!(key.starts_with(KEY_PREFIX));
        assert!(!key.contains("secret"));
        assert_eq!(key, storage_key("secret"));
    }
}
