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
    pub runtime_session_namespace: Option<String>,
    pub runtime_session_key_hint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodexAppHandshakeMeta {
    pub session_namespace: String,
    pub session_key_hint: String,
    pub freshness_seconds: Option<i64>,
}

#[derive(Clone)]
pub struct CodexAppAdapter {
    dsn: String,
    ws_client: Option<Arc<OpenClawWsClient>>,
}

impl CodexAppAdapter {
    pub fn new(dsn: String, ws_client: Option<Arc<OpenClawWsClient>>) -> Self {
        Self { dsn, ws_client }
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
        let handshake = CodexAppSessionSource::from_env(self.dsn.clone()).resolve(ctx);
        let payload = client
            .proxy_codex_app_chat(
                model,
                user_text,
                &handshake.session_namespace,
                &handshake.session_key_hint,
                handshake.freshness_seconds,
            )
            .await?;
        let runtime_namespace = extract_runtime_hint(&payload, "session_namespace");
        let runtime_key_hint = extract_runtime_hint(&payload, "session_key_hint");
        let runtime_source = describe_runtime_hint_source(&payload);
        extract_chat_text(&payload).map(|text| {
            format!(
                "codex-app-adapter child={} observed_at={} source={} page={} session_namespace={} session_key_hint={} freshness_seconds={} runtime_source={} runtime_session_namespace={} runtime_session_key_hint={} via openclaw-ws output={}",
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
                runtime_source,
                runtime_namespace.unwrap_or_else(|| "none".to_string()),
                runtime_key_hint.unwrap_or_else(|| "none".to_string()),
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
        let handshake = CodexAppSessionSource::from_env(self.dsn.clone()).resolve(ctx);
        let payload = client
            .proxy_codex_app_response(
                model,
                input,
                &handshake.session_namespace,
                &handshake.session_key_hint,
                handshake.freshness_seconds,
            )
            .await?;
        let runtime_namespace = extract_runtime_hint(&payload, "session_namespace");
        let runtime_key_hint = extract_runtime_hint(&payload, "session_key_hint");
        let runtime_source = describe_runtime_hint_source(&payload);
        extract_response_text(&payload).map(|text| {
            format!(
                "codex-app-adapter child={} observed_at={} source={} page={} session_namespace={} session_key_hint={} freshness_seconds={} runtime_source={} runtime_session_namespace={} runtime_session_key_hint={} via openclaw-ws output={}",
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
                runtime_source,
                runtime_namespace.unwrap_or_else(|| "none".to_string()),
                runtime_key_hint.unwrap_or_else(|| "none".to_string()),
                text
            )
        })
    }
}

fn extract_runtime_hint(payload: &Value, field: &str) -> Option<String> {
    payload
        .get("runtime")
        .and_then(|v| v.get(field))
        .and_then(|v| v.as_str())
        .or_else(|| payload.get("handshake").and_then(|v| v.get(field)).and_then(|v| v.as_str()))
        .or_else(|| {
            payload
                .get("choices")
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("message"))
                .and_then(|v| v.get("runtime"))
                .and_then(|v| v.get(field))
                .and_then(|v| v.as_str())
        })
        .or_else(|| {
            payload
                .get("choices")
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("message"))
                .and_then(|v| v.get("handshake"))
                .and_then(|v| v.get(field))
                .and_then(|v| v.as_str())
        })
        .or_else(|| {
            payload
                .get("output")
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("content"))
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("runtime"))
                .and_then(|v| v.get(field))
                .and_then(|v| v.as_str())
        })
        .or_else(|| {
            payload
                .get("output")
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("content"))
                .and_then(|v| v.get(0))
                .and_then(|v| v.get("handshake"))
                .and_then(|v| v.get(field))
                .and_then(|v| v.as_str())
        })
        .map(|v| v.to_string())
}

fn describe_runtime_hint_source(payload: &Value) -> &'static str {
    if payload.get("runtime").is_some() {
        "top-level-runtime"
    } else if payload.get("handshake").is_some() {
        "top-level-handshake"
    } else if payload
        .get("choices")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("message"))
        .and_then(|v| v.get("runtime"))
        .is_some()
    {
        "chat-message-runtime"
    } else if payload
        .get("choices")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("message"))
        .and_then(|v| v.get("handshake"))
        .is_some()
    {
        "chat-message-handshake"
    } else if payload
        .get("output")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("runtime"))
        .is_some()
    {
        "response-content-runtime"
    } else if payload
        .get("output")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("handshake"))
        .is_some()
    {
        "response-content-handshake"
    } else {
        "none"
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
