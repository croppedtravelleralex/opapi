use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize)]
pub struct Choice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,
}

pub async fn create_chat_completion(
    State(_config): State<Config>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Json<ChatCompletionResponse> {
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let last_user_message = payload
        .messages
        .iter()
        .rev()
        .find(|m| m.role == "user")
        .map(|m| m.content.clone())
        .unwrap_or_else(|| "hello".to_string());

    let reply = format!(
        "placeholder response from sub2api-gateway: received '{}'",
        last_user_message
    );

    Json(ChatCompletionResponse {
        id: format!("chatcmpl-{}", created),
        object: "chat.completion".to_string(),
        created,
        model: payload.model,
        choices: vec![Choice {
            index: 0,
            message: ChatMessage {
                role: "assistant".to_string(),
                content: reply,
            },
            finish_reason: "stop".to_string(),
        }],
    })
}
