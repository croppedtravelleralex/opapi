use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{config::Config, error::gateway_error};

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
    State(config): State<Config>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Response {
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
    let upstream_url = format!("{}/v1/chat/completions", upstream.base_url);

    let response = match client
        .post(&upstream_url)
        .bearer_auth(upstream.api_key)
        .json(&payload)
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

    (status, Json(body)).into_response()
}
