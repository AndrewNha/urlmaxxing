mod bookmarks;
mod health;
mod models;
mod state;
mod users;

use anyhow::Result;
use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let state = AppState { pool };

    let app = Router::new()
        .merge(health::router())
        .nest("/users", users::router())
        .nest("/bookmarks", bookmarks::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
