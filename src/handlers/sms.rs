use axum::{
    extract::{Query, State},
    Json
};

use crate::{
    db, errors::AppError, models::sms::{NewSms, SmsListResponse, SmsPayload, SmsQuery, SmsResponse}, AppState
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

pub async fn get_sms_handler(
    State(state): State<AppState>,
    Query(params): Query<SmsQuery>,
) -> Result<Json<SmsListResponse>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    let (sms_list, total) = db::get_sms_by_device_with_filters(
        &state.db_pool,
        params.device_id,
        params.from,
        params.to,
        limit,
        offset,
    ).await?;

    Ok(Json(SmsListResponse {
        total,
        data: sms_list,
    }))
}
