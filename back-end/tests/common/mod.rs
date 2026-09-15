use axum::http::StatusCode;
use axum_test::TestServer;
use back_end::{AppState, RateLimitSettings, create_app, create_cors};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::{net::SocketAddr, num::NonZeroU32};
use tower_cookies::Cookie;

pub const JWT_SECRET: &str = "test-secret";
pub const PASSWORD: &str = "password123";

pub fn test_server(pool: PgPool) -> TestServer {
    let state = AppState {
        pool,
        jwt_secret: JWT_SECRET.to_string(),
        cookie_secure: false,
    };
    let rate_limit_settings = RateLimitSettings {
        general_per_minute: NonZeroU32::new(10_000).unwrap(),
        login_per_minute: NonZeroU32::new(10_000).unwrap(),
        register_per_hour: NonZeroU32::new(10_000).unwrap(),
    };
    let app = create_app(state, create_cors(), rate_limit_settings)
        .into_make_service_with_connect_info::<SocketAddr>();
    TestServer::new(app)
}

pub async fn create_user(server: &TestServer, username: &str, password: &str) -> Value {
    let response = server
        .post("/users")
        .json(&json!({ "username": username, "password": password }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json::<Value>()
}

pub async fn login(server: &TestServer, username: &str, password: &str) -> Cookie<'static> {
    let response = server
        .post("/auth/login")
        .json(&json!({ "username": username, "password": password }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    response
        .maybe_cookie("session")
        .expect("login should return a session cookie")
}

pub async fn authenticated_user(server: &TestServer, username: &str) -> (Value, Cookie<'static>) {
    let user = create_user(server, username, PASSWORD).await;
    let session_cookie = login(server, username, PASSWORD).await;
    (user, session_cookie)
}
