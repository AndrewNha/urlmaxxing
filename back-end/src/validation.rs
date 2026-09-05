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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(validate_username("medicinagod").is_ok());
    }

    #[test]
    fn test_empty_username() {
        assert!(
            validate_username("     ").is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Username is required".to_string()))
        );
    }

    #[test]
    fn test_not_alphanumeric_username() {
        assert!(
            validate_username("@-;~´[|ت").is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Username must contain only alphanumeric characters or underscores".to_string()))
        );
    }

    #[test]
    fn test_short_username() {
        assert!(
            validate_username("hh").is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Username must be between 3 and 30 characters".to_string()))
        );
    }

    #[test]
    fn test_valid_password() {
        assert!(validate_password("valid_password").is_ok());
    }

    #[test]
    fn test_short_password() {
        assert!(
            validate_password("1234567").is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Password must be at least 8 characters".to_string()))
        );
    }

    #[test]
    fn test_empty_password() {
        assert!(
            validate_password("").is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Password is required".to_string()))
        );
    }

    #[test]
    fn test_large_password() {
        assert!(
            validate_password(&"a".repeat(31).to_string()).is_err_and(|e| matches!(e, AppError::ValidationError(message) if message == "Password must be at most 30 bytes".to_string()))
        );
    }
}
