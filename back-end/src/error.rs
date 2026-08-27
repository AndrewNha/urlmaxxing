use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    // ex: username já cadastrado
    Conflict(String),
    InvalidCredentials,
    /// falha no hash de senha (bcrypt).
    Hashing,
    /// Qualquer outro erro de banco de dados não tratado especificamente.
    Database(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
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
            // Código "23505" = unique_violation no Postgres.
            if db_error.code().as_deref() == Some("23505") {
                return AppError::Conflict("Resource already exists".to_string());
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
