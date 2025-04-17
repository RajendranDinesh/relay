use axum::{
    extract::State,
    Json
};

use crate::{
    AppState,
    db,
    errors::AppError,
    models::sms::{NewSms, SmsPayload, SmsResponse}
};

pub async fn sms_handler(
    State(state): State<AppState>,
    Json(payload): Json<SmsPayload>,
) -> Result<Json<SmsResponse>, AppError> {
    let new_sms = NewSms {
        device_id: &payload.device_id,
        sender: &payload.sender,
        message: &payload.message,
    };

    let saved_sms = db::create_sms(&state.db_pool, &new_sms).await?;

    Ok(Json(SmsResponse { id: saved_sms.id }))
}
