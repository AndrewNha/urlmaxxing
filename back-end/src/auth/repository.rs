use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let query = "SELECT id, username, password_hash FROM users WHERE username = $1";

    let user = sqlx::query_as::<_, User>(query)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn find_token_version(pool: &PgPool, user_id: Uuid) -> Result<Option<i32>> {
    let query = "SELECT token_version FROM users WHERE id = $1";
    let token_version = sqlx::query_scalar::<_, i32>(query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(token_version)
}
