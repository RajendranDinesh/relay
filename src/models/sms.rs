use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Sms {
    pub id: Uuid,
    pub device_id: Uuid,
    pub sender: String,
    pub message: String,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewSms<'a> {
    pub device_id: &'a Uuid,
    pub sender: &'a str,
    pub message: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct SmsPayload {
    pub device_id: Uuid,
    pub sender: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SmsResponse {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SmsQuery {
    pub device_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct SmsListResponse {
    pub total: i64,
    pub data: Vec<Sms>,
}
