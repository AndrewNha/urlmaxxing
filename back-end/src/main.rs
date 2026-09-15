use anyhow::Result;
use back_end::{AppState, RateLimitSettings, create_app, create_cors};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, net::SocketAddr, num::NonZeroU32};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let jwt_secret = env::var("JWT_SECRET")?;
    let cookie_secure = env::var("COOKIE_SECURE")?.parse::<bool>()?;
    let rate_limit_settings = RateLimitSettings {
        general_per_minute: env::var("RATE_LIMIT_GENERAL_PER_MINUTE")?.parse::<NonZeroU32>()?,
        login_per_minute: env::var("RATE_LIMIT_LOGIN_PER_MINUTE")?.parse::<NonZeroU32>()?,
        register_per_hour: env::var("RATE_LIMIT_REGISTER_PER_HOUR")?.parse::<NonZeroU32>()?,
    };

    let state = AppState {
        pool,
        jwt_secret,
        cookie_secure,
    };
    let cors = create_cors();
    let app = create_app(state, cors, rate_limit_settings);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
