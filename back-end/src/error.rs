use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    // ex: username já cadastrado
    Conflict(String),
    InvalidCredentials,
    // falha no hash de senha (bcrypt)
    Hashing,
    // erro de token
    TokenGenerationError,
    Unauthorized,
    ValidationError(String),
    // erro genérico de banco de dados
    Database(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ),
            AppError::Hashing => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error processing password".to_string(),
            ),
            AppError::TokenGenerationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error generating token".to_string(),
            ),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::ValidationError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Database(e) => {
                eprintln!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_error) = &error {
            // codigo "23505" = unique_violation no postgres
            if db_error.code().as_deref() == Some("23505") {
                return match db_error.constraint() {
                    Some("users_username_key") => {
                        AppError::Conflict("Username already exists".to_string())
                    }
                    _ => AppError::Conflict("Resource already exists".to_string()),
                };
            }
        }

        AppError::Database(error.into())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_error: bcrypt::BcryptError) -> Self {
        AppError::Hashing
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Database(error)
    }
}
