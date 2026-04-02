use axum::{routing::get, Router};

use crate::{config::Config, routes};

pub fn build_router(config: Config) -> Router {
    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .route("/v1/models", get(routes::models::list_models))
        .with_state(config)
}
