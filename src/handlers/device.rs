use axum::{extract::State, Json};

use crate::{
    auth::middleware::AuthRequired,
    errors::AppError,
    AppState,
    db
};
use crate::models::device::{
    FindAllResponse,
    NewDevice,
    RegisterPayload,
    RegisterResponse
};

pub async fn register_device(
    auth_wrapper: AuthRequired,
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterResponse>, AppError> {
    let user = auth_wrapper.0;

    let new_device = NewDevice {
        device_name: payload.device_name.as_str(),
        user_id: &user.user_id,
    };

    let device = db::create_device(&state.db_pool, &new_device).await?;

    Ok(Json(RegisterResponse{ device_id: device.id }))
}

pub async fn find_all_user_devices(
    auth_wrapper: AuthRequired,
    State(state): State<AppState>,
) -> Result<Json<FindAllResponse>, AppError> {
    let user = auth_wrapper.0;

    let devices = db::find_user_devices(&state.db_pool, user.user_id).await?;

    Ok(Json(FindAllResponse { devices }))
}
