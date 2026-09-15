pub mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use common::{PASSWORD, authenticated_user, test_server};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower_cookies::Cookie;
use uuid::Uuid;

async fn create_bookmark(
    server: &TestServer,
    session_cookie: &Cookie<'static>,
    title: &str,
    url: &str,
) -> Value {
    let response = server
        .post("/bookmarks")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "title": title,
            "url": url,
            "tags": ["rust", "docs"],
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json::<Value>()
}

#[sqlx::test(migrations = "./migrations")]
async fn create_bookmark_returns_created_and_persists_it(pool: PgPool) {
    let server = test_server(pool.clone());
    let (user, session_cookie) = authenticated_user(&server, "username").await;
    let bookmark = create_bookmark(
        &server,
        &session_cookie,
        "Rust",
        "https://www.rust-lang.org",
    )
    .await;

    assert_eq!(bookmark["user_id"], user["id"]);
    assert_eq!(bookmark["title"], "Rust");
    assert_eq!(bookmark["url"], "https://www.rust-lang.org/");
    assert_eq!(bookmark["tags"], json!(["rust", "docs"]));
    assert!(bookmark["created_at"].is_string());

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks WHERE id = $1")
        .bind(Uuid::parse_str(bookmark["id"].as_str().unwrap()).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migrations")]
async fn create_bookmark_without_session_returns_unauthorized(pool: PgPool) {
    let server = test_server(pool.clone());

    let response = server
        .post("/bookmarks")
        .json(&json!({
            "title": "Rust",
            "url": "https://www.rust-lang.org",
            "tags": ["rust"],
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.json::<Value>(), json!({ "error": "Unauthorized" }));

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migrations")]
async fn create_bookmark_rejects_invalid_url(pool: PgPool) {
    let server = test_server(pool.clone());
    let (_, session_cookie) = authenticated_user(&server, "username").await;

    for url in ["not-a-url", "ftp://example.com"] {
        let response = server
            .post("/bookmarks")
            .add_cookie(session_cookie.clone())
            .json(&json!({ "title": "Invalid", "url": url, "tags": [] }))
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.json::<Value>(), json!({ "error": "Invalid URL" }));
    }

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migrations")]
async fn list_bookmarks_returns_only_authenticated_users_items(pool: PgPool) {
    let server = test_server(pool);
    let (_, first_session_cookie) = authenticated_user(&server, "first_user").await;
    let first = create_bookmark(
        &server,
        &first_session_cookie,
        "First",
        "https://one.example",
    )
    .await;
    let second = create_bookmark(
        &server,
        &first_session_cookie,
        "Second",
        "https://two.example",
    )
    .await;

    let (_, other_session_cookie) = authenticated_user(&server, "other_user").await;
    create_bookmark(
        &server,
        &other_session_cookie,
        "Other",
        "https://other.example",
    )
    .await;

    let response = server
        .get("/bookmarks")
        .add_cookie(first_session_cookie)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let bookmarks = response.json::<Value>();
    let bookmarks = bookmarks.as_array().unwrap();
    assert_eq!(bookmarks.len(), 2);
    assert!(bookmarks.iter().any(|item| item["id"] == first["id"]));
    assert!(bookmarks.iter().any(|item| item["id"] == second["id"]));
}

#[sqlx::test(migrations = "./migrations")]
async fn get_bookmark_returns_owned_item_and_hides_other_users_item(pool: PgPool) {
    let server = test_server(pool);
    let (_, owner_session_cookie) = authenticated_user(&server, "owner").await;
    let bookmark = create_bookmark(
        &server,
        &owner_session_cookie,
        "Owned",
        "https://owned.example",
    )
    .await;
    let (_, other_session_cookie) = authenticated_user(&server, "other").await;
    let bookmark_id = bookmark["id"].as_str().unwrap();

    let owned_response = server
        .get(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(owner_session_cookie)
        .await;
    assert_eq!(owned_response.status_code(), StatusCode::OK);
    let owned = owned_response.json::<Value>();
    assert_eq!(owned["id"], bookmark["id"]);
    assert_eq!(owned["user_id"], bookmark["user_id"]);
    assert_eq!(owned["title"], bookmark["title"]);
    assert_eq!(owned["url"], bookmark["url"]);
    assert_eq!(owned["tags"], bookmark["tags"]);

    let other_response = server
        .get(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(other_session_cookie)
        .await;
    assert_eq!(other_response.status_code(), StatusCode::NOT_FOUND);
    assert_eq!(
        other_response.json::<Value>(),
        json!({ "error": "Resource not found" })
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn patch_bookmark_updates_only_sent_fields(pool: PgPool) {
    let server = test_server(pool);
    let (_, session_cookie) = authenticated_user(&server, "username").await;
    let bookmark = create_bookmark(&server, &session_cookie, "Old", "https://old.example").await;
    let bookmark_id = bookmark["id"].as_str().unwrap();

    let response = server
        .patch(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(session_cookie)
        .json(&json!({ "title": "New" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let updated = response.json::<Value>();
    assert_eq!(updated["title"], "New");
    assert_eq!(updated["url"], bookmark["url"]);
    assert_eq!(updated["tags"], bookmark["tags"]);
    assert_eq!(updated["id"], bookmark["id"]);
    assert_eq!(updated["user_id"], bookmark["user_id"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn replace_bookmark_updates_all_mutable_fields(pool: PgPool) {
    let server = test_server(pool);
    let (_, session_cookie) = authenticated_user(&server, "username").await;
    let bookmark = create_bookmark(&server, &session_cookie, "Old", "https://old.example").await;
    let bookmark_id = bookmark["id"].as_str().unwrap();

    let response = server
        .put(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(session_cookie)
        .json(&json!({
            "title": "New",
            "url": "https://new.example/path",
            "tags": ["new"],
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let replaced = response.json::<Value>();
    assert_eq!(replaced["id"], bookmark["id"]);
    assert_eq!(replaced["user_id"], bookmark["user_id"]);
    assert_eq!(replaced["title"], "New");
    assert_eq!(replaced["url"], "https://new.example/path");
    assert_eq!(replaced["tags"], json!(["new"]));
}

#[sqlx::test(migrations = "./migrations")]
async fn delete_bookmark_removes_it(pool: PgPool) {
    let server = test_server(pool.clone());
    let (_, session_cookie) = authenticated_user(&server, "username").await;
    let bookmark =
        create_bookmark(&server, &session_cookie, "Delete", "https://delete.example").await;
    let bookmark_id = bookmark["id"].as_str().unwrap();

    let response = server
        .delete(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(session_cookie.clone())
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let deleted = response.json::<Value>();
    assert_eq!(deleted["id"], bookmark["id"]);
    assert_eq!(deleted["user_id"], bookmark["user_id"]);
    assert_eq!(deleted["title"], bookmark["title"]);
    assert_eq!(deleted["url"], bookmark["url"]);
    assert_eq!(deleted["tags"], bookmark["tags"]);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks WHERE id = $1")
        .bind(Uuid::parse_str(bookmark_id).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);

    let get_response = server
        .get(&format!("/bookmarks/{bookmark_id}"))
        .add_cookie(session_cookie)
        .await;
    assert_eq!(get_response.status_code(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migrations")]
async fn deleting_user_cascades_to_bookmarks(pool: PgPool) {
    let server = test_server(pool.clone());
    let (user, session_cookie) = authenticated_user(&server, "username").await;
    create_bookmark(
        &server,
        &session_cookie,
        "Cascade",
        "https://cascade.example",
    )
    .await;
    let user_id = user["id"].as_str().unwrap();

    let response = server
        .delete(&format!("/users/{user_id}"))
        .add_cookie(session_cookie)
        .json(&json!({ "current_password": PASSWORD }))
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
