use crate::{
    middleware::{auth::auth_middleware, request_id::request_id_middleware},
    routes::{health, models},
    state::AppState,
};
use axum::{
    middleware,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/v1/models", get(models::list_models))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
