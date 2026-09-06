mod auth;
mod bookmarks;
mod error;
mod health;
mod models;
mod state;
mod users;
mod validation;

use axum::Router;
use axum::http::HeaderValue;
use axum::http::Method;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
pub use state::AppState;
use tower_http::cors::CorsLayer;

pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
}

pub fn create_app(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        .merge(health::router())
        .nest("/users", users::router())
        .nest("/bookmarks", bookmarks::router())
        .nest("/auth", auth::router())
        .layer(cors)
        .with_state(state)
}
