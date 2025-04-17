use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// Represents a user record fetched from the database
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewDevice<'a> {
    pub device_name: &'a str,
    pub user_id: &'a Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub device_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub device_id: Uuid,
}
