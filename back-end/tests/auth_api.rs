pub mod common;

use axum::http::StatusCode;
use common::{PASSWORD, create_user, test_server};
use serde_json::{Value, json};
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn login_returns_token_and_public_user(pool: PgPool) {
    let server = test_server(pool);
    let user = create_user(&server, "username", PASSWORD).await;

    let response = server
        .post("/auth/login")
        .json(&json!({ "username": "username", "password": PASSWORD }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body = response.json::<Value>();
    assert!(
        body["token"]
            .as_str()
            .is_some_and(|token| !token.is_empty())
    );
    assert_eq!(body["user"], user);
    assert!(body["user"].get("password").is_none());
    assert!(body["user"].get("password_hash").is_none());
}

#[sqlx::test(migrations = "./migrations")]
async fn login_normalizes_username(pool: PgPool) {
    let server = test_server(pool);
    create_user(&server, "username", PASSWORD).await;

    let response = server
        .post("/auth/login")
        .json(&json!({
            "username": "  USERNAME  ",
            "password": PASSWORD,
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[sqlx::test(migrations = "./migrations")]
async fn login_with_wrong_password_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool);
    create_user(&server, "username", PASSWORD).await;

    let response = server
        .post("/auth/login")
        .json(&json!({
            "username": "username",
            "password": "wrong_password",
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.json::<Value>(),
        json!({ "error": "Invalid username or password" })
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn login_with_unknown_user_returns_same_unauthorized_error(pool: PgPool) {
    let server = test_server(pool);

    let response = server
        .post("/auth/login")
        .json(&json!({
            "username": "unknown",
            "password": PASSWORD,
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.json::<Value>(),
        json!({ "error": "Invalid username or password" })
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn login_with_incomplete_json_returns_unprocessable_entity(pool: PgPool) {
    let server = test_server(pool);

    let response = server
        .post("/auth/login")
        .json(&json!({ "username": "username" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrations = "./migrations")]
async fn tampered_token_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool);
    let user = create_user(&server, "username", PASSWORD).await;
    let login_response = server
        .post("/auth/login")
        .json(&json!({ "username": "username", "password": PASSWORD }))
        .await;
    let mut token = login_response.json::<Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();
    token.push('x');

    let response = server
        .get(&format!("/users/{}", user["id"].as_str().unwrap()))
        .authorization_bearer(token)
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.json::<Value>(), json!({ "error": "Unauthorized" }));
}
