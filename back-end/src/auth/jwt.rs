use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::validated_token::ValidatedToken;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
    token_version: i32,
}

impl Claims {
    fn new(user_id: Uuid, token_version: i32) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            sub: user_id,
            exp: now + 300,
            token_version,
        }
    }
}

pub fn generate_token(
    user_id: Uuid,
    token_version: i32,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, token_version);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_token(
    token: &str,
    secret: &str,
) -> Result<ValidatedToken, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(ValidatedToken {
        user_id: (data.claims.sub),
        token_version: (data.claims.token_version),
    })
}
