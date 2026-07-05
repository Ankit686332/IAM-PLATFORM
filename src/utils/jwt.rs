use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn create_token(
    user_id: &str,
    secret: &str,
    hours: i64,
) -> String {
    let expiration = Utc::now() + Duration::hours(hours);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn generate_access_token(
    user_id: &str,
    secret: &str,
) -> String {
    create_token(user_id, secret, 24)
}

pub fn generate_refresh_token(
    user_id: &str,
    secret: &str,
) -> String {
    create_token(user_id, secret, 24 * 30)
}

pub fn verify_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}