use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::Config;

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
    let upstream_base_url = match &config.upstream_base_url {
        Some(v) => v.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": {
                        "message": "missing UPSTREAM_BASE_URL",
                        "type": "configuration_error"
                    }
                })),
            )
                .into_response()
        }
    };

    let upstream_api_key = match &config.upstream_api_key {
        Some(v) => v.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": {
                        "message": "missing UPSTREAM_API_KEY",
                        "type": "configuration_error"
                    }
                })),
            )
                .into_response()
        }
    };

    let client = reqwest::Client::new();
    let upstream_url = format!("{}/v1/chat/completions", upstream_base_url);

    let response = match client
        .post(&upstream_url)
        .bearer_auth(upstream_api_key)
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(err) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({
                    "error": {
                        "message": format!("upstream request failed: {}", err),
                        "type": "upstream_request_error"
                    }
                })),
            )
                .into_response()
        }
    };

    let status = response.status();
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({
                    "error": {
                        "message": format!("failed to decode upstream response: {}", err),
                        "type": "upstream_decode_error"
                    }
                })),
            )
                .into_response()
        }
    };

    (status, Json(body)).into_response()
}
