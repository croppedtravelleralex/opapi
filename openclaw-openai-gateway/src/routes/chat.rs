use crate::{
    codex::pool_router::PoolRouter,
    error::AppError,
    governance::audit::routing_event,
    observability::explain::explain,
    providers::ProviderAdapter,
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
    let user_text = payload
        .messages
        .last()
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let decision = decide_provider(&payload.model, &default_policy(), Some(state.as_ref()));
    let explain_text = explain(&decision);
    let audit = routing_event(&decision);
    state.audit_repo.append(&audit);

    let pool_router = PoolRouter::new(state.config.sqlite_path.clone());
    let routed_member = pool_router
        .pick_best_active_member()
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NoHealthyPoolMember)?;

    let result = match state.gateway_provider.chat(&payload.model, &user_text).await {
        Ok(result) => result,
        Err(_) => {
            return Ok(([
                ("x-routing-explain", explain_text),
                ("x-audit-action", audit.action),
                ("x-pool-child-account-id", routed_member.child_account_id),
                ("x-pool-admission-level", routed_member.admission_level),
            ], AppError::UpstreamUnavailable).into_response())
        }
    };

    Ok(([
        ("x-routing-explain", explain_text),
        ("x-audit-action", audit.action),
        ("x-pool-child-account-id", routed_member.child_account_id),
        ("x-pool-admission-level", routed_member.admission_level),
    ], Json(result)).into_response())
}
