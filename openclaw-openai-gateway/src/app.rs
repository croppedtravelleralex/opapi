use crate::{
    middleware::{auth::auth_middleware, request_id::request_id_middleware},
    routes::{chat, codex, health, models, ops, providers, responses},
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
        .route("/ops/overview", get(ops::get_ops_overview))
        .route("/ops/dashboard", get(ops::get_ops_dashboard))
        .route("/ops/scheduler/tick", post(ops::run_scheduler_tick))
        .route("/v1/codex/quota-sources", get(codex::list_codex_quota_sources))
        .route("/v1/codex/quota-overview", get(codex::get_codex_quota_overview))
        .route("/v1/codex/automation-targets/discover", post(codex::discover_automation_targets))
        .route("/v1/codex/automation-targets/try", post(codex::try_automation_target))
        .route("/v1/codex/auto-register", post(codex::auto_register_codex_account))
        .route("/v1/mailboxes/import", post(codex::import_managed_mailboxes))
        .route("/v1/mailboxes/overview", get(codex::get_mailbox_pool_overview))
        .route("/v1/mailboxes/expand", post(codex::expand_mailbox_pool))
        .route("/v1/mailboxes/tiering/run", post(codex::run_mailbox_tiering))
        .route("/v1/mailboxes/poll/run", post(codex::poll_managed_mailboxes))
        .route("/v1/codex/auto-register/autoloop/run", post(codex::run_registration_autoloop))
        .route("/v1/codex/auto-register/dispatch", post(codex::dispatch_registration_task))
        .route("/v1/codex/auto-register/dead-letter/recover", post(codex::recover_dead_letters))
        .route("/v1/codex/auto-register/worker/run", post(codex::run_registration_worker))
        .route("/v1/codex/quota/collect", post(codex::collect_codex_quota))
        .route("/v1/chat/completions", post(chat::create_chat_completion))
        .route("/v1/responses", post(responses::create_response))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
