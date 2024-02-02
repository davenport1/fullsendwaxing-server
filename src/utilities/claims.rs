use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use crate::utilities::fsw_error::FswError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
    roles: Vec<String>, // user roles
}

pub fn create_token() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::hours(1);
    now += expires_in;

    let claim = Claims {
        exp: now.timestamp() as usize,
        iat,
        roles: vec!["user".to_string()],
    };

    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_token_valid(token: &str) -> Result<bool, FswError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    _ = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|error| {
            match error.kind() {
                ErrorKind::ExpiredSignature => FswError::new(StatusCode::UNAUTHORIZED, "Your session has expired! Please login again."),
                _ => FswError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, please try again later.")
            }
        })?;
    Ok(true)
}