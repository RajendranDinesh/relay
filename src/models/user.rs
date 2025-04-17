use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Represents a user record fetched from the database
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)] // Don't send hash to client
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Structure for creating a new user in the DB
#[derive(Debug)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

// Payload for user registration request
#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

// Payload for user login request
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

// Response containing the JWT after successful login
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
}

// Represents the authenticated user's info extracted from JWT
// Often simpler than the full DB User struct
#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
   pub user_id: Uuid,
}