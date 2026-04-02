use axum::{routing::{get, post}, Router};

use crate::{config::Config, routes};

pub fn build_router(config: Config) -> Router {
    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .route("/v1/models", get(routes::models::list_models))
        .route("/v1/chat/completions", post(routes::chat::create_chat_completion))
        .with_state(config)
}
