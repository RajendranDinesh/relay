use axum::{extract::State, Json};
use crate::models::user::{RegisterPayload, LoginPayload, LoginResponse};
use crate::db;
use crate::auth::{password, jwt};
use crate::errors::AppError;
use crate::AppState;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<LoginResponse>, AppError> {

    if payload.username.is_empty() || payload.password.len() == 0 {
        return Err(AppError::BadRequest("Invalid name or password too short".to_string()));
    }

    let hashed_password = password::hash_password(&payload.password)?;

    let new_user_data = crate::models::user::NewUser {
        username: &payload.username,
        password_hash: &hashed_password,
    };

    let user = db::create_user(&state.db_pool, &new_user_data).await?;

    let token = jwt::create_jwt(user.id, &state.config.jwt_secret, state.config.jwt_expiration_seconds)?;
    Ok(Json(LoginResponse { token, token_type: "Bearer".to_string() }))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
     if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AppError::BadRequest("Username and password are required".to_string()));
    }

    let user = db::find_user_by_name(&state.db_pool, &payload.username)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let is_password_valid = password::verify_password(&payload.password, &user.password_hash)?;

    if !is_password_valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = jwt::create_jwt(user.id, &state.config.jwt_secret, state.config.jwt_expiration_seconds)?;

    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}
