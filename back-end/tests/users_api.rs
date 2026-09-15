pub mod common;

use axum::http::StatusCode;
use bcrypt::verify;
use common::{PASSWORD, create_user, login, test_server};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "./migrations")]
async fn health_returns_ok(pool: PgPool) {
    let response = test_server(pool).get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[sqlx::test(migrations = "./migrations")]
async fn create_user_returns_created_and_persists_hashed_password(pool: PgPool) {
    let server = test_server(pool.clone());
    let response = server
        .post("/users")
        .json(&json!({
            "username": "  Username  ",
            "password": PASSWORD,
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body = response.json::<Value>();
    assert_eq!(body["username"], "username");
    assert!(body.get("password").is_none());
    assert!(body.get("password_hash").is_none());

    let password_hash: String =
        sqlx::query_scalar("SELECT password_hash FROM users WHERE username = $1")
            .bind("username")
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_ne!(password_hash, PASSWORD);
    assert!(verify(PASSWORD, &password_hash).unwrap());
}

#[sqlx::test(migrations = "./migrations")]
async fn duplicate_username_returns_conflict(pool: PgPool) {
    let server = test_server(pool);
    create_user(&server, "username", "PASSWORD").await;

    let response = server
        .post("/users")
        .json(&json!({ "username": "USERNAME", "password": "PASSWORD" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
    assert_eq!(
        response.json::<Value>(),
        json!({ "error": "Username already exists" })
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn invalid_user_returns_bad_request_and_is_not_persisted(pool: PgPool) {
    let server = test_server(pool.clone());
    let response = server
        .post("/users")
        .json(&json!({ "username": "ab", "password": "short" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_user_returns_authenticated_user(pool: PgPool) {
    let server = test_server(pool);
    let user = create_user(&server, "username", PASSWORD).await;
    let session_cookie = login(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();

    let response = server
        .get(&format!("/users/{user_id}"))
        .add_cookie(session_cookie)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.json::<Value>(), user);
}

#[sqlx::test(migrations = "./migrations")]
async fn get_user_without_session_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool);
    let user = create_user(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();

    let response = server.get(&format!("/users/{user_id}")).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.json::<Value>(), json!({ "error": "Unauthorized" }));
}

#[sqlx::test(migrations = "./migrations")]
async fn get_another_user_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool);
    create_user(&server, "username", PASSWORD).await;
    let session_cookie = login(&server, "username", PASSWORD).await;

    let response = server
        .get(&format!("/users/{}", Uuid::new_v4()))
        .add_cookie(session_cookie)
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "./migrations")]
async fn replace_user_updates_username(pool: PgPool) {
    let server = test_server(pool.clone());
    let user = create_user(&server, "username", PASSWORD).await;
    let session_cookie = login(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();

    let response = server
        .put(&format!("/users/{user_id}"))
        .add_cookie(session_cookie)
        .json(&json!({ "username": "  New_Username  " }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.json::<Value>()["username"], "new_username");

    let username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
        .bind(Uuid::parse_str(user_id).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(username, "new_username");
}

#[sqlx::test(migrations = "./migrations")]
async fn update_password_replaces_hash_and_invalidates_old_session(pool: PgPool) {
    let server = test_server(pool.clone());
    let user = create_user(&server, "username", PASSWORD).await;
    let old_session_cookie = login(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();
    let new_password = "new_password123";

    let response = server
        .patch(&format!("/users/{user_id}/password"))
        .add_cookie(old_session_cookie.clone())
        .json(&json!({
            "current_password": PASSWORD,
            "new_password": new_password,
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let old_session_response = server
        .get(&format!("/users/{user_id}"))
        .add_cookie(old_session_cookie)
        .await;
    assert_eq!(old_session_response.status_code(), StatusCode::UNAUTHORIZED);

    let password_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = $1")
        .bind(Uuid::parse_str(user_id).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(verify(new_password, &password_hash).unwrap());

    login(&server, "username", new_password).await;
}

#[sqlx::test(migrations = "./migrations")]
async fn update_password_with_wrong_current_password_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool);
    let user = create_user(&server, "username", PASSWORD).await;
    let session_cookie = login(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();

    let response = server
        .patch(&format!("/users/{user_id}/password"))
        .add_cookie(session_cookie)
        .json(&json!({
            "current_password": "wrong_password",
            "new_password": "new_password123",
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.json::<Value>(),
        json!({ "error": "Invalid username or password" })
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn delete_user_removes_authenticated_user(pool: PgPool) {
    let server = test_server(pool.clone());
    let user = create_user(&server, "username", PASSWORD).await;
    let session_cookie = login(&server, "username", PASSWORD).await;
    let user_id = user["id"].as_str().unwrap();

    let response = server
        .delete(&format!("/users/{user_id}"))
        .add_cookie(session_cookie)
        .json(&json!({ "current_password": PASSWORD }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.json::<Value>(), user);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id = $1")
        .bind(Uuid::parse_str(user_id).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
