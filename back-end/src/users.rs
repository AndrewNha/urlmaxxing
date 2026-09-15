mod handlers;
mod repository;

use axum::{
    Router,
    routing::{get, patch, post},
};
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp};
use std::num::NonZeroU32;

use crate::state::AppState;

pub fn router(register_per_hour: NonZeroU32) -> Router<AppState> {
    let register_rate_limit_config = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_hour(register_per_hour))
        .finish()
        .expect("invalid registration rate limit configuration");

    let register_router = Router::new()
        .route("/", post(handlers::create_user))
        .layer(GovernorLayer::new(register_rate_limit_config));

    Router::new()
        .merge(register_router)
        .route(
            "/{id}",
            get(handlers::get_user)
                .put(handlers::replace_user)
                .delete(handlers::delete_user),
        )
        .route("/{id}/password", patch(handlers::update_password))
}
