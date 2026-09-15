pub(crate) mod cookie;
mod extractor;
mod handlers;
mod jwt;
mod repository;
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp};
use std::num::NonZeroU32;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub fn router(login_per_minute: NonZeroU32) -> Router<AppState> {
    let login_rate_limit_config =
        GovernorConfigBuilder::default() // config vazia
            .with_extractor(PeerIp::default()) // chave usada para separar os clientes
            .expect_connect_info() // PeerIp precisa que o Axum forneça o endereço da conexão
            .quota_default(Quota::requests_per_minute(login_per_minute))
            .finish()
            .expect("invalid login rate limit configuration");

    let login_router = Router::new()
        .route("/login", post(handlers::login))
        .layer(GovernorLayer::new(login_rate_limit_config));

    Router::new()
        .merge(login_router)
        .route("/logout", post(handlers::logout))
        .route("/me", get(handlers::me))
}
