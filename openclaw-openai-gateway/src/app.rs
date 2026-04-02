use crate::{
    middleware::{auth::auth_middleware, request_id::request_id_middleware},
    routes::{chat, health, models, providers, responses},
    state::AppState,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/v1/models", get(models::list_models))
        .route("/v1/providers", get(providers::list_providers))
        .route("/v1/chat/completions", post(chat::create_chat_completion))
        .route("/v1/responses", post(responses::create_response))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
