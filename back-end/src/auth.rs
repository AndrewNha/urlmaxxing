mod handlers;
mod jwt;
mod repository;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(handlers::login))
}
