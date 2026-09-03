use crate::error::AppError;

pub(crate) fn validate_username(username: &str) -> Result<(), AppError> {
    if username.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Username is required".to_string(),
        ));
    }
    if username.trim().chars().count() > 30 || username.trim().chars().count() < 3 {
        return Err(AppError::ValidationError(
            "Username must be between 3 and 30 characters".to_string(),
        ));
    }
    let has_only_valid_characters = username
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_');
    if !has_only_valid_characters {
        return Err(AppError::ValidationError(
            "Username must contain only alphanumeric characters or underscores".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_password(password: &str) -> Result<(), AppError> {
    if password.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Password is required".to_string(),
        ));
    }
    if password.chars().count() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    if password.len() > 30 {
        return Err(AppError::ValidationError(
            "Password must be at most 30 bytes".to_string(),
        ));
    }
    Ok(())
}
