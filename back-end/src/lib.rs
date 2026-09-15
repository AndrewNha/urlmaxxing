mod auth;
mod bookmarks;
mod config;
mod error;
mod health;
mod models;
mod state;
mod users;
mod validation;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp};
pub use config::RateLimitSettings;
pub use state::AppState;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub fn create_cors() -> CorsLayer {
    let frontend_origin = std::env::var("FRONTEND_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".into())
        .parse::<HeaderValue>()
        .expect("invalid FRONTEND_ORIGIN");

    let local_origin = HeaderValue::from_static("http://127.0.0.1:5173");

    CorsLayer::new()
        .allow_origin([frontend_origin, local_origin])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true)
}

pub fn create_app(
    state: AppState,
    cors: CorsLayer,
    rate_limit_settings: RateLimitSettings,
) -> Router {
    let cookie = CookieManagerLayer::new();

    let general_rate_limit_config = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_minute(
            rate_limit_settings.general_per_minute,
        ))
        .finish()
        .expect("invalid general rate limit configuration");

    let api_router = Router::new()
        .nest(
            "/users",
            users::router(rate_limit_settings.register_per_hour),
        )
        .nest("/bookmarks", bookmarks::router())
        .nest("/auth", auth::router(rate_limit_settings.login_per_minute))
        .layer(GovernorLayer::new(general_rate_limit_config));

    Router::new()
        .merge(health::router())
        .merge(api_router)
        .layer(cookie)
        .layer(cors)
        .with_state(state)
}
