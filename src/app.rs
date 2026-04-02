use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{auth, config::Config, routes};

pub fn build_router(config: Config) -> Router {
    let protected_routes = Router::new()
        .route("/v1/models", get(routes::models::list_models))
        .route("/v1/chat/completions", post(routes::chat::create_chat_completion))
        .layer(middleware::from_fn_with_state(
            config.clone(),
            auth::require_bearer_auth,
        ));

    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .merge(protected_routes)
        .with_state(config)
}
