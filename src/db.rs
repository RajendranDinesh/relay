use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::AppError;
use crate::models::user::{User, NewUser};

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
