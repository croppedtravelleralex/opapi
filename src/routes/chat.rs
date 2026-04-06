use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    app::AppState,
    error::{gateway_error, normalize_upstream_error},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

pub async fn create_chat_completion(
    State(state): State<AppState>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Response {
    let config = &state.config;

    let upstream = match config.upstream_for_model(&payload.model) {
        Some(v) => v,
        None => {
            return gateway_error(
                StatusCode::BAD_REQUEST,
                format!("no upstream configured for model '{}'", payload.model),
                "routing_error",
            )
        }
    };

    let client = reqwest::Client::new();
    let upstream_model = if upstream.name == "date-now" {
        "date-now-gpt-5.4".to_string()
    } else {
        payload.model.clone()
    };
    let upstream_url = if upstream.append_v1 {
        format!("{}/v1/chat/completions", upstream.base_url)
    } else {
        format!("{}/chat/completions", upstream.base_url)
    };

    let upstream_payload = serde_json::json!({
        "model": upstream_model,
        "messages": payload.messages,
        "temperature": payload.temperature,
        "max_tokens": payload.max_tokens,
        "stream": payload.stream
    });

    let response = match client
        .post(&upstream_url)
        .bearer_auth(upstream.api_key)
        .json(&upstream_payload)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(err) => {
            return gateway_error(
                StatusCode::BAD_GATEWAY,
                format!("upstream request failed: {}", err),
                "upstream_request_error",
            )
        }
    };

    let status = response.status();
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => {
            return gateway_error(
                StatusCode::BAD_GATEWAY,
                format!("failed to decode upstream response: {}", err),
                "upstream_decode_error",
            )
        }
    };

    if !status.is_success() {
        return normalize_upstream_error(status, body);
    }

    (status, Json(body)).into_response()
}
