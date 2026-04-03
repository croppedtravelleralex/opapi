use crate::bridge::client::OpenClawWsClient;
use crate::codex::codex_app_session_source::CodexAppSessionSource;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CodexAppRequestContext {
    pub child_account_id: String,
    pub source_id: String,
    pub source_page: String,
    pub observed_at: String,
}

#[derive(Debug, Clone)]
pub struct CodexAppHandshakeMeta {
    pub session_namespace: String,
    pub session_key_hint: String,
    pub freshness_seconds: Option<i64>,
}

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
        ctx: &CodexAppRequestContext,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let handshake = CodexAppSessionSource::from_env().resolve(ctx);
        let payload = client
            .proxy_codex_app_chat(
                model,
                user_text,
                &handshake.session_namespace,
                &handshake.session_key_hint,
                handshake.freshness_seconds,
            )
            .await?;
        extract_chat_text(&payload).map(|text| {
            format!(
                "codex-app-adapter child={} observed_at={} source={} page={} session_namespace={} session_key_hint={} freshness_seconds={} via openclaw-ws output={}",
                ctx.child_account_id,
                ctx.observed_at,
                ctx.source_id,
                ctx.source_page,
                handshake.session_namespace,
                handshake.session_key_hint,
                handshake
                    .freshness_seconds
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                text
            )
        })
    }

    pub async fn run_response_via_ws(
        &self,
        ctx: &CodexAppRequestContext,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        let client = self
            .ws_client
            .as_ref()
            .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
        let handshake = CodexAppSessionSource::from_env().resolve(ctx);
        let payload = client
            .proxy_codex_app_response(
                model,
                input,
                &handshake.session_namespace,
                &handshake.session_key_hint,
                handshake.freshness_seconds,
            )
            .await?;
        extract_response_text(&payload).map(|text| {
            format!(
                "codex-app-adapter child={} observed_at={} source={} page={} session_namespace={} session_key_hint={} freshness_seconds={} via openclaw-ws output={}",
                ctx.child_account_id,
                ctx.observed_at,
                ctx.source_id,
                ctx.source_page,
                handshake.session_namespace,
                handshake.session_key_hint,
                handshake
                    .freshness_seconds
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                text
            )
        })
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
