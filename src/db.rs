use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::sms::{NewSms, Sms};
use crate::models::user::{User, NewUser};
use crate::models::device::{NewDevice, Device};

pub async fn create_user(pool: &PgPool, new_user: &NewUser<'_>) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, password_hash, created_at, updated_at
        "#,
        new_user.username,
        new_user.password_hash,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
         if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::UserAlreadyExists;
            }
        }
        AppError::DatabaseError(e)
    })?;

    Ok(user)
}

pub async fn find_user_by_name(pool: &PgPool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, AppError> {
     let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(user)
}

pub async fn create_device(pool: &PgPool, new_device: &NewDevice<'_>) -> Result<Device, AppError> {
    let device = sqlx::query_as!(
        Device,
        r#"
        INSERT INTO devices (user_id, device_name)
        VALUES ($1, $2)
        RETURNING id, user_id, device_name, created_at, updated_at
        "#,
        new_device.user_id,
        new_device.device_name,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Failed to insert device: {:?}", e);
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::DeviceAlreadyExists;
            }
        }
        AppError::DatabaseError(e)
    })?;

    Ok(device)
}

pub async fn find_user_devices(pool: &PgPool, user_id: Uuid) -> Result<Vec<Device>, AppError> {
    let devices = sqlx::query_as!(
       Device,
       r#"
       SELECT id, device_name, user_id, created_at, updated_at
       FROM devices
       WHERE user_id = $1
       "#,
       user_id
   )
   .fetch_all(pool)
   .await
   .map_err(AppError::DatabaseError)?;

   Ok(devices)
}

pub async fn create_sms(pool: &PgPool, sms: &NewSms<'_>) -> Result<Sms, AppError> {
    let sms = sqlx::query_as!(
        Sms,
        r#"
        INSERT INTO sms (device_id, sender, message)
        VALUES ($1, $2, $3)
        RETURNING id, device_id, sender, message, received_at
        "#,
        sms.device_id,
        sms.sender,
        sms.message
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(sms)
}

pub async fn get_sms_by_device_with_filters(
    pool: &PgPool,
    device_id: Uuid,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Sms>, i64), AppError> {
    let rows = sqlx::query_as!(
        Sms,
        r#"
        SELECT id, device_id, sender, message, received_at
        FROM sms
        WHERE device_id = $1
        AND ($2::timestamptz IS NULL OR received_at >= $2)
        AND ($3::timestamptz IS NULL OR received_at <= $3)
        ORDER BY received_at DESC
        LIMIT $4 OFFSET $5
        "#,
        device_id,
        from,
        to,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM sms
        WHERE device_id = $1
        AND ($2::timestamptz IS NULL OR received_at >= $2)
        AND ($3::timestamptz IS NULL OR received_at <= $3)
        "#,
        device_id,
        from,
        to,
    )
    .fetch_one(pool)
    .await?;

    Ok((rows, total.unwrap_or(0)))
}
