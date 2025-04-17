use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::AppError;

// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(AppError::PasswordHashingError)
}

// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(AppError::PasswordHashingError)
}
