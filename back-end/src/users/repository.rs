use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    replace_user_request::ReplaceUserRequest, update_profile_request::UpdateProfileRequest,
    user::User, user_response::UserResponse,
};

pub async fn insert_user(pool: &PgPool, user: &User) -> Result<()> {
    let query =
        "INSERT INTO users (id, display_name, username, password_hash) VALUES ($1, $2, $3, $4)";

    sqlx::query(query)
        .bind(user.id())
        .bind(user.display_name())
        .bind(user.username())
        .bind(user.password_hash())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_users(pool: &PgPool) -> Result<Vec<UserResponse>> {
    let users = sqlx::query_as::<_, UserResponse>("SELECT id, display_name, username FROM users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn find_user(pool: &PgPool, id: &Uuid) -> Result<Option<UserResponse>> {
    let user = sqlx::query_as::<_, UserResponse>(
        "SELECT id, display_name, username FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn save_user(
    pool: &PgPool,
    user_id: &Uuid,
    req: &UpdateProfileRequest,
) -> Result<Option<UserResponse>> {
    let query = "UPDATE users
         SET display_name = COALESCE($1, display_name),
             username = COALESCE($2, username)
         WHERE id = $3
         RETURNING id, display_name, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(&req.display_name)
        .bind(&req.username)
        .bind(user_id)
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
         SET display_name = $1,
             username = $2
         WHERE id = $3
         RETURNING id, display_name, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(&req.display_name)
        .bind(&req.username)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn remove_user(pool: &PgPool, user_id: &Uuid) -> Result<Option<UserResponse>> {
    let query = "DELETE FROM users WHERE id = $1 RETURNING id, display_name, username";

    let user = sqlx::query_as::<_, UserResponse>(query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
