use crate::bridge::client::OpenClawWsClient;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct CodexSessionBridge {
    mode: String,
    ws_client: Option<Arc<OpenClawWsClient>>,
}

impl CodexSessionBridge {
    pub fn new(mode: String, ws_client: Option<Arc<OpenClawWsClient>>) -> Self {
        Self { mode, ws_client }
    }

    pub async fn run_chat(
        &self,
        source_id: &str,
        source_page: &str,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        match self.mode.as_str() {
            "mock" => Ok(format!(
                "mock-session-bridge source={} page={} input={}",
                source_id, source_page, user_text
            )),
            "openclaw-ws" => {
                let client = self
                    .ws_client
                    .as_ref()
                    .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
                let payload = client.proxy_chat(model, user_text).await?;
                extract_chat_text(&payload).map(|text| {
                    format!(
                        "openclaw-ws-session-bridge source={} page={} output={}",
                        source_id, source_page, text
                    )
                })
            }
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
    }

    pub async fn run_response(
        &self,
        source_id: &str,
        source_page: &str,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        match self.mode.as_str() {
            "mock" => Ok(format!(
                "mock-session-bridge source={} page={} input={}",
                source_id, source_page, input
            )),
            "openclaw-ws" => {
                let client = self
                    .ws_client
                    .as_ref()
                    .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
                let payload = client.proxy_response(model, input).await?;
                extract_response_text(&payload).map(|text| {
                    format!(
                        "openclaw-ws-session-bridge source={} page={} output={}",
                        source_id, source_page, text
                    )
                })
            }
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
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
