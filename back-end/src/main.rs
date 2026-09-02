mod auth;
mod bookmarks;
mod error;
mod health;
mod models;
mod state;
mod users;

use anyhow::Result;
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tower_http::cors::CorsLayer;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let jwt_secret = env::var("JWT_SECRET")?;

    let state = AppState { pool, jwt_secret };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let app = Router::new()
        .merge(health::router())
        .nest("/users", users::router())
        .nest("/bookmarks", bookmarks::router())
        .nest("/auth", auth::router())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
