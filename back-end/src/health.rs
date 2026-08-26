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
