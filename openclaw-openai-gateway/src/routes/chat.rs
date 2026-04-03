use crate::{
    codex::{executor::CodexExecutor, pool_router::PoolRouter},
    error::AppError,
    governance::audit::routing_event,
    observability::explain::explain,
    routing::policy::{decide_provider, default_policy},
    state::AppState,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub async fn create_chat_completion(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Result<Response, AppError> {
    let decision = decide_provider(&payload.model, &default_policy(), Some(state.as_ref()));
    let explain_text = explain(&decision);
    let audit = routing_event(&decision);
    state.audit_repo.append(&audit);

    let pool_router = PoolRouter::new(state.config.sqlite_path.clone());
    let member = pool_router
        .pick_best_active_member()
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NoHealthyPoolMember)?;

    let user_text = payload
        .messages
        .last()
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let executor = CodexExecutor::new(
        state.config.sqlite_path.clone(),
        state.config.codex_session_bridge_mode.clone(),
    );
    let result = executor
        .execute_chat(&member, &payload.model, &user_text)
        .await
        .map_err(|_| AppError::UpstreamUnavailable)?;

    Ok(([
        ("x-routing-explain", explain_text),
        ("x-audit-action", audit.action),
        ("x-pool-child-account-id", member.child_account_id),
        ("x-pool-admission-level", member.admission_level),
    ], Json(result))
        .into_response())
}
