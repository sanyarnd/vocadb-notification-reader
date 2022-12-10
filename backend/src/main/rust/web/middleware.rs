pub mod auth_token {
    use std::ops::Add;

    use actix_web::dev::ServiceRequest;
    use actix_web_httpauth::extractors::bearer::BearerAuth;
    use aes::Aes128;
    use anyhow::Context;
    use awc::cookie::Cookie;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use hex_literal::hex;

    use crate::service::notifications::Database;
    use crate::web::dto::Token;
    use crate::web::errors::Result;

    type Aes = Cbc<Aes128, Pkcs7>;

    const KEY: [u8; 16] = hex!("a0b0c0d0f0a1b4c5f0f9a9a9a1c2d4ff");
    const IV: [u8; 16] = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");

    pub fn parse(token: &str) -> Result<Token> {
        let raw = base64::decode(token).context("Token is not in base64 format")?;

        let aes_cipher: Aes = Aes::new_from_slices(&KEY, &IV).unwrap();
        let token = aes_cipher
            .decrypt_vec(&raw)
            .context("Unable to decode a token")?;
        let token_json = serde_json::from_slice(&token).context("Unable to deserialize a token")?;

        Ok(token_json)
    }

    pub fn encode(user_id: i32, database: &Database, cookies: &[Cookie]) -> Result<String> {
        let exp_date = chrono::Utc::now().add(chrono::Duration::weeks(1));
        let token = Token {
            user_id,
            database: database.clone(),
            cookies: cookies.iter().map(|c| c.to_string()).collect(),
            exp: exp_date.timestamp(),
        };

        let token = serde_json::to_vec(&token).context("Unable to serialize a token")?;

        let aes_cipher: Aes = Aes::new_from_slices(&KEY, &IV).unwrap();
        let data = aes_cipher.encrypt_vec(&token);

        Ok(base64::encode(&data))
    }

    pub async fn validate(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> actix_web::Result<ServiceRequest, (actix_web::Error, ServiceRequest)>  {
        let token = credentials.token();
        if let Err(err) = parse(token) {
            return Err((err.into(), req))
        }

        Ok(req)
    }
}
