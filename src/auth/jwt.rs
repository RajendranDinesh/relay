use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // Subject (user ID)
    pub exp: i64,  // Expiration time (timestamp)
    pub iat: i64,  // Issued at (timestamp)
}

pub fn create_jwt(user_id: Uuid, secret: &str, expiration_seconds: i64) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(expiration_seconds);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&header, &claims, &encoding_key)
        .map_err(|e| AppError::JwtError(e.into()))
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| AppError::JwtError(e.into()))
}
