use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{auth, config::Config, routes, store::AccountStore};

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
        .route("/v1/accounts/import", post(routes::accounts::import_accounts))
        .route("/v1/accounts/status", post(routes::accounts::update_account_status))
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
