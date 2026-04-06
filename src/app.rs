use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{auth, routes, store::AccountStore, config::Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub store: AccountStore,
}

pub fn build_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .route("/v1/models", get(routes::models::list_models))
        .route("/v1/chat/completions", post(routes::chat::create_chat_completion))
        .route("/v1/accounts", get(routes::accounts::list_accounts))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_bearer_auth,
        ));

    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .merge(protected_routes)
        .with_state(state)
}
