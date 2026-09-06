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
    InvalidUrl,
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
            AppError::InvalidUrl => (StatusCode::BAD_REQUEST, "Invalid URL".to_string()),
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

fn classify_database_error(code: Option<&str>, constraint: Option<&str>) -> Option<AppError> {
    // codigo "23505" = unique_violation no postgres
    if code != Some("23505") {
        return None;
    }

    Some(match constraint {
        Some("users_username_key") => AppError::Conflict("Username already exists".to_string()),
        _ => AppError::Conflict("Resource already exists".to_string()),
    })
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_error) = &error
            && let Some(app_error) =
                classify_database_error(db_error.code().as_deref(), db_error.constraint())
        {
            return app_error;
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

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use serde_json::{Value, from_slice};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::*;

    #[tokio::test]
    async fn test_not_found_returns_expected_error() {
        let response = AppError::NotFound.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Resource not found" }));
    }

    #[tokio::test]
    async fn test_conflict_returns_expected_error() {
        let response = AppError::Conflict("Username already exists".to_string()).into_response();

        assert_eq!(response.status(), StatusCode::CONFLICT);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Username already exists" }));
    }

    #[tokio::test]
    async fn test_unauthorized_returns_expected_error() {
        let response = AppError::Unauthorized.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Unauthorized" }));
    }

    #[tokio::test]
    async fn test_validation_error_returns_expected_error() {
        let response = AppError::ValidationError("Invalid input".to_string()).into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Invalid input" }));
    }

    #[tokio::test]
    async fn test_invalid_url_returns_expected_error() {
        let response = AppError::InvalidUrl.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Invalid URL" }));
    }

    #[tokio::test]
    async fn test_database_error_returns_expected_error() {
        let response = AppError::Database(anyhow::anyhow!("Database error")).into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Internal server error" }));
    }

    #[tokio::test]
    async fn test_hashing_error_returns_expected_error() {
        let response = AppError::Hashing.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Error processing password" }));
    }

    #[tokio::test]
    async fn test_token_generation_error_returns_expected_error() {
        let response = AppError::TokenGenerationError.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Error generating token" }));
    }

    #[tokio::test]
    async fn test_invalid_credentials_returns_expected_error() {
        let response = AppError::InvalidCredentials.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        let value: Value = from_slice(&bytes).unwrap();

        assert_eq!(value, json!({ "error": "Invalid username or password" }));
    }

    #[test]
    fn unique_username_violation_becomes_username_conflict() {
        let error = classify_database_error(Some("23505"), Some("users_username_key"));

        assert!(matches!(
            error,
            Some(AppError::Conflict(message))
                if message == "Username already exists"
        ));
    }

    #[test]
    fn other_unique_violation_becomes_generic_conflict() {
        let error = classify_database_error(Some("23505"), Some("bookmarks_url_key"));

        assert!(matches!(
            error,
            Some(AppError::Conflict(message))
                if message == "Resource already exists"
        ));
    }

    #[test]
    fn non_unique_violation_is_not_classified_as_conflict() {
        let error = classify_database_error(Some("23503"), None);

        assert!(error.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn unique_username_returns_conflict(pool: PgPool) {
        sqlx::query(
            r#"
              INSERT INTO users (id, username, password_hash)
              VALUES ($1, $2, $3)
              "#,
        )
        .bind(Uuid::new_v4())
        .bind("andre")
        .bind("hash")
        .execute(&pool)
        .await
        .unwrap();

        let sqlx_error = sqlx::query(
            r#"
              INSERT INTO users (id, username, password_hash)
              VALUES ($1, $2, $3)
              "#,
        )
        .bind(Uuid::new_v4())
        .bind("andre")
        .bind("another_hash")
        .execute(&pool)
        .await
        .unwrap_err();

        let app_error = AppError::from(sqlx_error);

        assert!(matches!(
            app_error,
            AppError::Conflict(message)
                if message == "Username already exists"
        ));
    }
}
