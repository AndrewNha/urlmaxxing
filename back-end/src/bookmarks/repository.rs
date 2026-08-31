use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    bookmark::Bookmark, bookmark_request::CreateBookmarkRequest,
    update_bookmark_request::UpdateBookmarkRequest,
};

pub async fn insert_bookmark(pool: &PgPool, bookmark: &Bookmark) -> Result<()> {
    let query = "INSERT INTO bookmarks (id, user_id, title, url, tags, created_at) VALUES ($1, $2, $3, $4, $5, $6)";

    sqlx::query(query)
        .bind(bookmark.id())
        .bind(bookmark.user_id())
        .bind(bookmark.title())
        .bind(bookmark.url())
        .bind(bookmark.tags())
        .bind(bookmark.created_at())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_bookmarks(pool: &PgPool, user_id: &Uuid) -> Result<Vec<Bookmark>> {
    let bookmarks = sqlx::query_as::<_, Bookmark>(
        "SELECT id, user_id, title, url, tags, created_at FROM bookmarks WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(bookmarks)
}

pub async fn find_bookmark(pool: &PgPool, id: &Uuid, user_id: &Uuid) -> Result<Option<Bookmark>> {
    let bookmark = sqlx::query_as::<_, Bookmark>(
        "SELECT id, user_id, title, url, tags, created_at FROM bookmarks WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(bookmark)
}

pub async fn save_bookmark(
    pool: &PgPool,
    id: &Uuid,
    req: &UpdateBookmarkRequest,
    user_id: &Uuid,
) -> Result<Option<Bookmark>> {
    let query = "UPDATE bookmarks
         SET title = COALESCE($1, title),
             url = COALESCE($2, url),
             tags = COALESCE($3, tags)
         WHERE id = $4 AND user_id = $5
         RETURNING id, user_id, title, url, tags, created_at";

    let bookmark = sqlx::query_as::<_, Bookmark>(query)
        .bind(&req.title)
        .bind(&req.url)
        .bind(&req.tags)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(bookmark)
}

pub async fn replace_bookmark(
    pool: &PgPool,
    id: &Uuid,
    req: &CreateBookmarkRequest,
    user_id: &Uuid,
) -> Result<Option<Bookmark>> {
    let query = "UPDATE bookmarks
         SET title = $1,
             url = $2,
             tags = $3
         WHERE id = $4 AND user_id = $5
         RETURNING id, user_id, title, url, tags, created_at";

    let bookmark = sqlx::query_as::<_, Bookmark>(query)
        .bind(&req.title)
        .bind(&req.url)
        .bind(&req.tags)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(bookmark)
}

pub async fn remove_bookmark(pool: &PgPool, id: &Uuid, user_id: &Uuid) -> Result<Option<Bookmark>> {
    let query = "DELETE FROM bookmarks WHERE id = $1 AND user_id = $2 RETURNING id, user_id, title, url, tags, created_at";

    let bookmark = sqlx::query_as::<_, Bookmark>(query)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(bookmark)
}
