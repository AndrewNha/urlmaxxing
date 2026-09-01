use anyhow::Result;
use sqlx::PgPool;

use crate::models::user::User;

pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let query = "SELECT id, username, password_hash FROM users WHERE username = $1";

    let user = sqlx::query_as::<_, User>(query)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
