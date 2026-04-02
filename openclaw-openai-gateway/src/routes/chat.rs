use crate::{
    error::AppError,
    observability::explain::explain,
    providers::ProviderAdapter,
    routing::policy::{decide_provider, default_policy},
    state::AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
) -> Result<impl IntoResponse, AppError> {
    let user_text = payload
        .messages
        .last()
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let decision = decide_provider(&payload.model, &default_policy());
    let explain_text = explain(&decision);

    let result = state
        .gateway_provider
        .chat(&payload.model, &user_text)
        .await
        .map_err(|_| AppError::UpstreamUnavailable)?;

    Ok(([ ("x-routing-explain", explain_text) ], Json(result)))
}
