use crate::{
    bridge::client::OpenClawWsClient,
    codex::codex_app_adapter::{CodexAppAdapter, CodexAppRequestContext},
};
use serde_json::Value;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct CodexSessionBridge {
    mode: String,
    dsn: String,
    ws_client: Option<Arc<OpenClawWsClient>>,
}

#[derive(Clone, Copy)]
enum CodexSourceAdapter {
    App,
    Web,
}

impl CodexSessionBridge {
    pub fn new(mode: String, dsn: String, ws_client: Option<Arc<OpenClawWsClient>>) -> Self {
        Self { mode, dsn, ws_client }
    }

    pub async fn run_chat(
        &self,
        child_account_id: &str,
        source_id: &str,
        source_page: &str,
        observed_at: &str,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        let adapter = resolve_source_adapter(source_id)?;
        match self.mode.as_str() {
            "mock" => self.mock_chat(adapter, source_id, source_page, user_text).await,
            "openclaw-ws" => {
                self.openclaw_ws_chat(
                    adapter,
                    child_account_id,
                    source_id,
                    source_page,
                    observed_at,
                    model,
                    user_text,
                )
                .await
            }
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
    }

    pub async fn run_response(
        &self,
        child_account_id: &str,
        source_id: &str,
        source_page: &str,
        observed_at: &str,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        let adapter = resolve_source_adapter(source_id)?;
        match self.mode.as_str() {
            "mock" => self.mock_response(adapter, source_id, source_page, input).await,
            "openclaw-ws" => {
                self.openclaw_ws_response(
                    adapter,
                    child_account_id,
                    source_id,
                    source_page,
                    observed_at,
                    model,
                    input,
                )
                .await
            }
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
        child_account_id: &str,
        source_id: &str,
        source_page: &str,
        observed_at: &str,
        model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        match adapter {
            CodexSourceAdapter::App => {
                let app = CodexAppAdapter::new(self.dsn.clone(), self.ws_client.clone());
                let mut ctx = build_codex_app_request_context(
                    child_account_id,
                    source_id,
                    source_page,
                    observed_at,
                );
                if let Some(client) = self.ws_client.as_ref() {
                    let payload = client
                        .proxy_codex_app_chat(
                            model,
                            user_text,
                            ctx.runtime_session_namespace
                                .as_deref()
                                .unwrap_or("codex-app-runtime-pending"),
                            ctx.runtime_session_key_hint
                                .as_deref()
                                .unwrap_or("codex-app-runtime-pending"),
                            None,
                        )
                        .await?;
                    apply_runtime_hints_from_payload(&mut ctx, &payload);
                }
                app.run_chat_via_ws(&ctx, model, user_text).await
            }
            CodexSourceAdapter::Web => {
                let client = self
                    .ws_client
                    .as_ref()
                    .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
                let payload = client.proxy_chat(model, user_text).await?;
                let runtime_namespace = find_runtime_hint(&payload, "session_namespace")
                    .unwrap_or_else(|| "none".to_string());
                let runtime_key_hint = find_runtime_hint(&payload, "session_key_hint")
                    .unwrap_or_else(|| "none".to_string());
                extract_chat_text(&payload).map(|text| {
                    format!(
                        "openclaw-ws-session-bridge adapter={} source={} page={} runtime_source={} runtime_session_namespace={} runtime_session_key_hint={} output={}",
                        adapter_name(adapter),
                        source_id,
                        source_page,
                        describe_runtime_hint_source(&payload),
                        runtime_namespace,
                        runtime_key_hint,
                        text
                    )
                })
            }
        }
    }

    async fn openclaw_ws_response(
        &self,
        adapter: CodexSourceAdapter,
        child_account_id: &str,
        source_id: &str,
        source_page: &str,
        observed_at: &str,
        model: &str,
        input: &str,
    ) -> Result<String, String> {
        match adapter {
            CodexSourceAdapter::App => {
                let app = CodexAppAdapter::new(self.dsn.clone(), self.ws_client.clone());
                let mut ctx = build_codex_app_request_context(
                    child_account_id,
                    source_id,
                    source_page,
                    observed_at,
                );
                if let Some(client) = self.ws_client.as_ref() {
                    let payload = client
                        .proxy_codex_app_response(
                            model,
                            input,
                            ctx.runtime_session_namespace
                                .as_deref()
                                .unwrap_or("codex-app-runtime-pending"),
                            ctx.runtime_session_key_hint
                                .as_deref()
                                .unwrap_or("codex-app-runtime-pending"),
                            None,
                        )
                        .await?;
                    apply_runtime_hints_from_payload(&mut ctx, &payload);
                }
                app.run_response_via_ws(&ctx, model, input).await
            }
            CodexSourceAdapter::Web => {
                let client = self
                    .ws_client
                    .as_ref()
                    .ok_or_else(|| "missing_openclaw_ws_client".to_string())?;
                let payload = client.proxy_response(model, input).await?;
                let runtime_namespace = find_runtime_hint(&payload, "session_namespace")
                    .unwrap_or_else(|| "none".to_string());
                let runtime_key_hint = find_runtime_hint(&payload, "session_key_hint")
                    .unwrap_or_else(|| "none".to_string());
                extract_response_text(&payload).map(|text| {
                    format!(
                        "openclaw-ws-session-bridge adapter={} source={} page={} runtime_source={} runtime_session_namespace={} runtime_session_key_hint={} output={}",
                        adapter_name(adapter),
                        source_id,
                        source_page,
                        describe_runtime_hint_source(&payload),
                        runtime_namespace,
                        runtime_key_hint,
                        text
                    )
                })
            }
        }
    }
}

fn build_codex_app_request_context(
    child_account_id: &str,
    source_id: &str,
    source_page: &str,
    observed_at: &str,
) -> CodexAppRequestContext {
    CodexAppRequestContext {
        child_account_id: child_account_id.to_string(),
        source_id: source_id.to_string(),
        source_page: source_page.to_string(),
        observed_at: observed_at.to_string(),
        runtime_session_namespace: resolve_runtime_session_env(
            "CODEX_APP_RUNTIME_SESSION_NAMESPACE",
            child_account_id,
            source_id,
        ),
        runtime_session_key_hint: resolve_runtime_session_env(
            "CODEX_APP_RUNTIME_SESSION_KEY_HINT",
            child_account_id,
            source_id,
        ),
    }
}

fn apply_runtime_hints_from_payload(ctx: &mut CodexAppRequestContext, payload: &Value) {
    if ctx.runtime_session_namespace.is_none() {
        ctx.runtime_session_namespace = find_runtime_hint(payload, "session_namespace");
    }
    if ctx.runtime_session_key_hint.is_none() {
        ctx.runtime_session_key_hint = find_runtime_hint(payload, "session_key_hint");
    }
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

fn find_runtime_hint(payload: &Value, field: &str) -> Option<String> {
    payload
        .get("runtime")
        .and_then(|v| v.get(field))
        .and_then(|v| v.as_str())
        .or_else(|| {
            payload
                .get("handshake")
                .and_then(|v| v.get(field))
                .and_then(|v| v.as_str())
        })
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

fn resolve_runtime_session_env(prefix: &str, child_account_id: &str, source_id: &str) -> Option<String> {
    let child = sanitize_env_fragment(child_account_id);
    let source = sanitize_env_fragment(source_id);
    let keys = [
        format!("{}_{}_{}", prefix, child, source),
        format!("{}_{}", prefix, child),
        prefix.to_string(),
    ];
    for key in keys {
        if let Ok(value) = env::var(&key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn sanitize_env_fragment(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch.to_ascii_uppercase() } else { '_' })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_codex_app_request_context_uses_scoped_runtime_env() {
        unsafe {
            env::set_var("CODEX_APP_RUNTIME_SESSION_NAMESPACE_CHILD_1_CODEX_APP", "scoped-ns");
            env::set_var("CODEX_APP_RUNTIME_SESSION_KEY_HINT_CHILD_1_CODEX_APP", "scoped-key");
        }

        let ctx = build_codex_app_request_context("child-1", "codex-app", "/codex", "2026-04-03T12:00:00+08:00");
        assert_eq!(ctx.runtime_session_namespace.as_deref(), Some("scoped-ns"));
        assert_eq!(ctx.runtime_session_key_hint.as_deref(), Some("scoped-key"));

        unsafe {
            env::remove_var("CODEX_APP_RUNTIME_SESSION_NAMESPACE_CHILD_1_CODEX_APP");
            env::remove_var("CODEX_APP_RUNTIME_SESSION_KEY_HINT_CHILD_1_CODEX_APP");
        }
    }

    #[test]
    fn build_codex_app_request_context_falls_back_to_global_runtime_env() {
        unsafe {
            env::set_var("CODEX_APP_RUNTIME_SESSION_NAMESPACE", "global-ns");
            env::set_var("CODEX_APP_RUNTIME_SESSION_KEY_HINT", "global-key");
        }

        let ctx = build_codex_app_request_context("child-2", "codex-app", "/codex", "2026-04-03T12:00:00+08:00");
        assert_eq!(ctx.runtime_session_namespace.as_deref(), Some("global-ns"));
        assert_eq!(ctx.runtime_session_key_hint.as_deref(), Some("global-key"));

        unsafe {
            env::remove_var("CODEX_APP_RUNTIME_SESSION_NAMESPACE");
            env::remove_var("CODEX_APP_RUNTIME_SESSION_KEY_HINT");
        }
    }

    #[test]
    fn apply_runtime_hints_from_payload_accepts_top_level_handshake() {
        let mut ctx = CodexAppRequestContext {
            child_account_id: "child-3".into(),
            source_id: "codex-app".into(),
            source_page: "/codex".into(),
            observed_at: "2026-04-03T12:00:00+08:00".into(),
            runtime_session_namespace: None,
            runtime_session_key_hint: None,
        };

        let payload = serde_json::json!({
            "handshake": {
                "session_namespace": "handshake-ns",
                "session_key_hint": "handshake-key"
            }
        });

        apply_runtime_hints_from_payload(&mut ctx, &payload);
        assert_eq!(ctx.runtime_session_namespace.as_deref(), Some("handshake-ns"));
        assert_eq!(ctx.runtime_session_key_hint.as_deref(), Some("handshake-key"));
    }

    #[test]
    fn apply_runtime_hints_from_payload_accepts_nested_response_handshake() {
        let mut ctx = CodexAppRequestContext {
            child_account_id: "child-4".into(),
            source_id: "codex-app".into(),
            source_page: "/codex".into(),
            observed_at: "2026-04-03T12:00:00+08:00".into(),
            runtime_session_namespace: None,
            runtime_session_key_hint: None,
        };

        let payload = serde_json::json!({
            "output": [{
                "content": [{
                    "handshake": {
                        "session_namespace": "nested-ns",
                        "session_key_hint": "nested-key"
                    }
                }]
            }]
        });

        apply_runtime_hints_from_payload(&mut ctx, &payload);
        assert_eq!(ctx.runtime_session_namespace.as_deref(), Some("nested-ns"));
        assert_eq!(ctx.runtime_session_key_hint.as_deref(), Some("nested-key"));
    }

    #[test]
    fn describe_runtime_hint_source_reports_nested_chat_runtime() {
        let payload = serde_json::json!({
            "choices": [{
                "message": {
                    "runtime": {
                        "session_namespace": "chat-ns",
                        "session_key_hint": "chat-key"
                    }
                }
            }]
        });

        assert_eq!(describe_runtime_hint_source(&payload), "chat-message-runtime");
    }
}
