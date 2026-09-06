use axum::http::StatusCode;
use axum_test::TestServer;
use back_end::{AppState, create_app, create_cors};
use serde_json::{Value, json};
use sqlx::PgPool;

pub const JWT_SECRET: &str = "test-secret";
pub const PASSWORD: &str = "password123";

pub fn test_server(pool: PgPool) -> TestServer {
    let state = AppState {
        pool,
        jwt_secret: JWT_SECRET.to_string(),
    };
    TestServer::new(create_app(state, create_cors()))
}

pub async fn create_user(server: &TestServer, username: &str, password: &str) -> Value {
    let response = server
        .post("/users")
        .json(&json!({ "username": username, "password": password }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json::<Value>()
}

pub async fn login(server: &TestServer, username: &str, password: &str) -> String {
    let response = server
        .post("/auth/login")
        .json(&json!({ "username": username, "password": password }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    response.json::<Value>()["token"]
        .as_str()
        .unwrap()
        .to_string()
}

pub async fn authenticated_user(server: &TestServer, username: &str) -> (Value, String) {
    let user = create_user(server, username, PASSWORD).await;
    let token = login(server, username, PASSWORD).await;
    (user, token)
}
