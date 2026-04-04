use crate::{
    middleware::{auth::auth_middleware, request_id::request_id_middleware},
    routes::{chat, codex, health, models, providers, responses},
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
        .route("/v1/codex/quota-sources", get(codex::list_codex_quota_sources))
        .route("/v1/codex/quota-overview", get(codex::get_codex_quota_overview))
        .route("/v1/codex/auto-register", post(codex::auto_register_codex_account))
        .route("/v1/codex/quota/collect", post(codex::collect_codex_quota))
        .route("/v1/chat/completions", post(chat::create_chat_completion))
        .route("/v1/responses", post(responses::create_response))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
