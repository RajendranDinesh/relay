use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader
};

use crate::errors::AppError;
use crate::auth::jwt;
use crate::models::user::AuthenticatedUser;
use crate::AppState;
use crate::db;

// Extractor that validates the JWT and provides AuthenticatedUser
#[derive(Debug, Clone)]
pub struct AuthRequired(pub AuthenticatedUser);

impl FromRequestParts<AppState> for AuthRequired
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?; // Use Unauthorized for missing/malformed header

        // Decode the user data
        let claims = jwt::validate_jwt(bearer.token(), &state.config.jwt_secret)
            .map_err(|_| AppError::Unauthorized)?;

        // Check if user still exists in DB - adds overhead but increases security
        if db::find_user_by_id(&state.db_pool, claims.sub).await?.is_none() {
            return Err(AppError::Unauthorized);
        }

        // Construct the authenticated user representation
        let auth_user = AuthenticatedUser { user_id: claims.sub };

        Ok(AuthRequired(auth_user))
    }
}
