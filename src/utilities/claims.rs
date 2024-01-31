use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_token() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::hours(1);
    now += expires_in;

    let claim = Claims {
        exp: now.timestamp() as usize,
        iat,
    };

    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_token_valid(token: &str) -> Result<bool, StatusCode> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    _ = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|error| {
            match error.kind() {
                ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;
    Ok(true)
}