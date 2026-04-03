use crate::bridge::client::OpenClawWsClient;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct CodexSessionBridge {
    mode: String,
    ws_client: Option<Arc<OpenClawWsClient>>,
}

#[derive(Clone, Copy)]
enum CodexSourceAdapter {
    App,
    Web,
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
        let adapter = resolve_source_adapter(source_id)?;
        match self.mode.as_str() {
            "mock" => self.mock_chat(adapter, source_id, source_page, user_text).await,
            "openclaw-ws" => self.openclaw_ws_chat(adapter, source_id, source_page, model, user_text).await,
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
        let adapter = resolve_source_adapter(source_id)?;
        match self.mode.as_str() {
            "mock" => self.mock_response(adapter, source_id, source_page, input).await,
            "openclaw-ws" => self.openclaw_ws_response(adapter, source_id, source_page, model, input).await,
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
    }

    async fn mock_chat(
        &self,
        adapter: CodexSourceAdapter,
        source_id: &str,
        source_page: &str,
        user_text: &str,
    ) -> Result<String, String> {
        Ok(format!(
            "mock-session-bridge adapter={} source={} page={} input={}",
            adapter_name(adapter),
            source_id,
            source_page,
            user_text
        ))
    }

    async fn mock_response(
        &self,
        adapter: CodexSourceAdapter,
        source_id: &str,
        source_page: &str,
        input: &str,
    ) -> Result<String, String> {
        Ok(format!(
            "mock-session-bridge adapter={} source={} page={} input={}",
            adapter_name(adapter),
            source_id,
            source_page,
            input
        ))
    }

    async fn openclaw_ws_chat(
        &self,
        adapter: CodexSourceAdapter,
        source_id: &str,
        source_page: &str,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let payload = client.proxy_chat(model, user_text).await?;
        extract_chat_text(&payload).map(|text| {
            format!(
                "openclaw-ws-session-bridge adapter={} source={} page={} output={}",
                adapter_name(adapter),
                source_id,
                source_page,
                text
            )
        })
    }

    async fn openclaw_ws_response(
        &self,
        adapter: CodexSourceAdapter,
        source_id: &str,
        source_page: &str,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let payload = client.proxy_response(model, input).await?;
        extract_response_text(&payload).map(|text| {
            format!(
                "openclaw-ws-session-bridge adapter={} source={} page={} output={}",
                adapter_name(adapter),
                source_id,
                source_page,
                text
            )
        })
    }
}

fn resolve_source_adapter(source_id: &str) -> Result<CodexSourceAdapter, String> {
    match source_id {
        "codex-app" => Ok(CodexSourceAdapter::App),
        "codex-web" => Ok(CodexSourceAdapter::Web),
        other => Err(format!("unsupported_codex_source_adapter:{}", other)),
    }
}

fn adapter_name(adapter: CodexSourceAdapter) -> &'static str {
    match adapter {
        CodexSourceAdapter::App => "codex-app",
        CodexSourceAdapter::Web => "codex-web",
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
