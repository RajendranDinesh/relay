use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] bcrypt::BcryptError),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("Unauthorized: Authentication required or token invalid")]
    Unauthorized,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Bad request: {0}")]
    BadRequest(String),

     #[error("Internal server error")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ConfigError(e) => {
                // Log the sensitive details, return a generic message
                error!("Configuration error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error".to_string())
            }
            AppError::DatabaseError(e) => {
                 error!("Database error: {:?}", e);
                 // Check for specific DB errors if needed, otherwise generic
                 (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed".to_string())
            }
            AppError::PasswordHashingError(e) => {
                error!("Password hashing error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::JwtError(e) => {
                 error!("JWT processing error: {:?}", e);
                 // Usually indicates a bad token from the client
                 (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string())
            }
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect email or password".to_string())
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Authentication required".to_string())
            }
             AppError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "Username is already in use".to_string()) 
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::InternalServerError(msg) => {
                 error!("Internal Server Error: {}", msg);
                 (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred".to_string())
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
