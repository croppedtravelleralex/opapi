use crate::{error::AppError, providers::ProviderAdapter, state::AppState};
use axum::{extract::State, Json};
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
) -> Result<Json<Value>, AppError> {
    let user_text = payload
        .messages
        .last()
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let result = state
        .gateway_provider
        .chat(&payload.model, &user_text)
        .await
        .map_err(|_| AppError::UpstreamUnavailable)?;

    Ok(Json(result))
}
