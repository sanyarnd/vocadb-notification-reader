//! Encrypted session token handed out to the frontend.
//!
//! The token carries VocaDB session cookies, so it is encrypted with AES-256-GCM.

use aes_gcm::aead::{Aead, Generate, Key, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::service::Database;

pub const TOKEN_TTL: Duration = Duration::weeks(1);
const NONCE_LEN: usize = 12;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub user_id: i32,
    pub database: Database,
    pub cookies: Vec<String>,
    pub exp: i64,
}

impl Token {
    pub fn new(user_id: i32, database: Database, cookies: Vec<String>, now: DateTime<Utc>) -> Self {
        Token {
            user_id,
            database,
            cookies,
            exp: (now + TOKEN_TTL).timestamp(),
        }
    }
}

#[derive(Clone)]
pub struct TokenCodec {
    cipher: Aes256Gcm,
}

impl TokenCodec {
    /// Creates a codec from a hex encoded 32-byte key.
    pub fn from_hex(key: &str) -> anyhow::Result<Self> {
        let key = hex::decode(key.trim()).context("Token key must be hex encoded")?;
        let key = Key::<Aes256Gcm>::try_from(key.as_slice())
            .map_err(|_| anyhow::anyhow!("Token key must be 32 bytes long"))?;
        Ok(TokenCodec {
            cipher: Aes256Gcm::new(&key),
        })
    }

    /// Creates a codec with a random key; tokens won't survive a restart.
    pub fn random() -> Self {
        TokenCodec {
            cipher: Aes256Gcm::new(&Key::<Aes256Gcm>::generate()),
        }
    }

    pub fn encode(&self, token: &Token) -> anyhow::Result<String> {
        let plaintext = serde_json::to_vec(token).context("Unable to serialize a token")?;
        let nonce = Nonce::generate();
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_slice())
            .map_err(|_| anyhow::anyhow!("Unable to encrypt a token"))?;

        let mut data = nonce.to_vec();
        data.extend_from_slice(&ciphertext);
        Ok(URL_SAFE_NO_PAD.encode(data))
    }

    pub fn decode(&self, token: &str, now: DateTime<Utc>) -> anyhow::Result<Token> {
        let data = URL_SAFE_NO_PAD
            .decode(token)
            .context("Token is not in base64 format")?;
        if data.len() <= NONCE_LEN {
            bail!("Token is too short");
        }

        let (nonce, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = Nonce::try_from(nonce).map_err(|_| anyhow::anyhow!("Invalid token nonce"))?;
        let plaintext = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Unable to decrypt a token"))?;
        let token: Token =
            serde_json::from_slice(&plaintext).context("Unable to deserialize a token")?;

        if token.exp < now.timestamp() {
            bail!("Token has expired");
        }

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(now: DateTime<Utc>) -> Token {
        Token::new(
            42,
            Database::TouhouDb,
            vec![".AspNetCore.Cookies=abc".to_string()],
            now,
        )
    }

    #[test]
    fn round_trip() {
        let codec = TokenCodec::random();
        let now = Utc::now();
        let original = token(now);

        let encoded = codec.encode(&original).unwrap();
        assert_eq!(codec.decode(&encoded, now).unwrap(), original);
    }

    #[test]
    fn same_token_encrypts_differently() {
        let codec = TokenCodec::random();
        let token = token(Utc::now());
        assert_ne!(codec.encode(&token).unwrap(), codec.encode(&token).unwrap());
    }

    #[test]
    fn rejects_expired_token() {
        let codec = TokenCodec::random();
        let now = Utc::now();
        let encoded = codec.encode(&token(now)).unwrap();

        assert!(
            codec
                .decode(&encoded, now + TOKEN_TTL - Duration::seconds(1))
                .is_ok()
        );
        assert!(
            codec
                .decode(&encoded, now + TOKEN_TTL + Duration::seconds(1))
                .is_err()
        );
    }

    #[test]
    fn rejects_token_from_another_key() {
        let now = Utc::now();
        let encoded = TokenCodec::random().encode(&token(now)).unwrap();
        assert!(TokenCodec::random().decode(&encoded, now).is_err());
    }

    #[test]
    fn rejects_tampered_token() {
        let codec = TokenCodec::random();
        let now = Utc::now();
        let encoded = codec.encode(&token(now)).unwrap();

        let mut data = URL_SAFE_NO_PAD.decode(&encoded).unwrap();
        *data.last_mut().unwrap() ^= 1;
        let tampered = URL_SAFE_NO_PAD.encode(data);

        assert!(codec.decode(&tampered, now).is_err());
        assert!(codec.decode("not base64!", now).is_err());
        assert!(codec.decode("", now).is_err());
    }

    #[test]
    fn key_from_hex() {
        let key = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let now = Utc::now();
        let encoded = TokenCodec::from_hex(key)
            .unwrap()
            .encode(&token(now))
            .unwrap();
        assert!(
            TokenCodec::from_hex(key)
                .unwrap()
                .decode(&encoded, now)
                .is_ok()
        );

        assert!(TokenCodec::from_hex("0011").is_err());
        assert!(TokenCodec::from_hex("zz").is_err());
    }
}
