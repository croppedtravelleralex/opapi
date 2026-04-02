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

fn gateway_error(status: StatusCode, message: impl Into<String>, err_type: &str) -> Response {
    (
        status,
        Json(json!({
            "error": {
                "message": message.into(),
                "type": err_type
            }
        })),
    )
        .into_response()
}

pub async fn create_chat_completion(
    State(config): State<Config>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Response {
    let upstream_base_url = match &config.upstream_base_url {
        Some(v) => v.clone(),
        None => return gateway_error(StatusCode::BAD_REQUEST, "missing UPSTREAM_BASE_URL", "configuration_error"),
    };

    let upstream_api_key = match &config.upstream_api_key {
        Some(v) => v.clone(),
        None => return gateway_error(StatusCode::BAD_REQUEST, "missing UPSTREAM_API_KEY", "configuration_error"),
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
