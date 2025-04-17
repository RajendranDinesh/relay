use std::env;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_seconds: i64,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
    #[error("Database connection failed: {0}")]
    DatabaseConnection(sqlx::Error),
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok(); // Load .env file if present

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingVar("JWT_SECRET".to_string()))?;
        let jwt_expiration_seconds = env::var("JWT_EXPIRATION_SECONDS")
            .unwrap_or_else(|_| "3600".to_string()) // Default to 1 hour
            .parse::<i64>()
            .map_err(|e| ConfigError::InvalidValue("JWT_EXPIRATION_SECONDS".to_string(), e.to_string()))?;

        Ok(AppConfig {
            database_url,
            jwt_secret,
            jwt_expiration_seconds,
        })
    }
}

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, ConfigError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(ConfigError::DatabaseConnection)
}