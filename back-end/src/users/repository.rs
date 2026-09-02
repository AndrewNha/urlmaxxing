use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    replace_user_request::ReplaceUserRequest, user::User, user_response::UserResponse,
};

pub async fn insert_user(pool: &PgPool, user: &User) -> Result<()> {
    let query = "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(user.id())
        .bind(user.username())
        .bind(user.password_hash())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_user(pool: &PgPool, id: &Uuid) -> Result<Option<UserResponse>> {
    let user = sqlx::query_as::<_, UserResponse>("SELECT id, username FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn replace_user(
    pool: &PgPool,
    user_id: &Uuid,
    req: &ReplaceUserRequest,
) -> Result<Option<UserResponse>> {
    let query = "UPDATE users
         SET username = $1
         WHERE id = $2
         RETURNING id, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(&req.username)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn remove_user(pool: &PgPool, user_id: &Uuid) -> Result<Option<UserResponse>> {
    let query = "DELETE FROM users WHERE id = $1 RETURNING id, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_password(
    pool: &PgPool,
    user_id: &Uuid,
    new_password_hash: &str,
) -> Result<Option<UserResponse>> {
    let query = "UPDATE users
         SET password_hash = $1
         WHERE id = $2
         RETURNING id, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(new_password_hash)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
