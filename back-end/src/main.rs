use anyhow::Result;
use back_end::{AppState, create_app, create_cors};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let jwt_secret = env::var("JWT_SECRET")?;

    let state = AppState { pool, jwt_secret };
    let cors = create_cors();
    let app = create_app(state, cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
