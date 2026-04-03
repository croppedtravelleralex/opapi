use crate::bridge::client::OpenClawWsClient;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct CodexAppAdapter {
    ws_client: Option<Arc<OpenClawWsClient>>,
}

impl CodexAppAdapter {
    pub fn new(ws_client: Option<Arc<OpenClawWsClient>>) -> Self {
        Self { ws_client }
    }

    pub async fn run_chat_via_ws(
        &self,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let payload = client.proxy_chat(model, user_text).await?;
        extract_chat_text(&payload)
            .map(|text| format!("codex-app-adapter via openclaw-ws output={}", text))
    }

    pub async fn run_response_via_ws(
        &self,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let payload = client.proxy_response(model, input).await?;
        extract_response_text(&payload)
            .map(|text| format!("codex-app-adapter via openclaw-ws output={}", text))
    }
}

fn extract_chat_text(payload: &Value) -> Result<String, String> {
    payload
        .get("choices")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("message"))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .ok_or_else(|| "invalid_openclaw_ws_chat_payload".to_string())
}

fn extract_response_text(payload: &Value) -> Result<String, String> {
    payload
        .get("output")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("text"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .ok_or_else(|| "invalid_openclaw_ws_response_payload".to_string())
}
