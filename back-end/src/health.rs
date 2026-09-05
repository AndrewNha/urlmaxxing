use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::{models::health_response::HealthResponse, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "OK".to_string(),
    };
    (StatusCode::OK, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use sqlx::pool::PoolOptions;

    #[tokio::test]
    async fn test_health_check() {
        let pool = PoolOptions::new()
            .connect_lazy("postgres://user:123@localhost:5432/database_test")
            .unwrap();
        let jwt_secret = "med".to_string();
        let state = AppState { pool, jwt_secret };

        let app = router().with_state(state);
        let client = TestServer::new(app);

        let response = client.get("/health").await;
        response.assert_status(StatusCode::OK);
        response.assert_json::<HealthResponse>(&HealthResponse {
            status: "OK".to_string(),
        });
    }
}
