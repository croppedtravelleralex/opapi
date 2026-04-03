use crate::bridge::{
    mapper::{
        map_chat_request,
        map_chat_response,
        map_codex_app_chat_request,
        map_codex_app_response_request,
        map_response_output,
        map_response_request,
    },
    types::{BridgeRequest, BridgeResponse},
};
use serde_json::Value;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;

pub struct OpenClawWsClient {
    url: String,
    timeout_ms: u64,
}

impl OpenClawWsClient {
    pub fn new(url: String, timeout_ms: u64) -> Self {
        Self { url, timeout_ms }
    }

    pub async fn check_ready(&self) -> bool {
        let connect = timeout(Duration::from_millis(self.timeout_ms), connect_async(&self.url)).await;
        matches!(connect, Ok(Ok((_ws, _resp))))
    }

    pub async fn proxy_chat(
        &self,
        model: &str,
        user_text: &str,
    ) -> Result<Value, String> {
        if !self.check_ready().await {
            return Err("upstream unavailable".into());
        }

        let bridge_request = BridgeRequest {
            upstream_payload: map_chat_request(model, user_text),
        };
        let assistant_text = bridge_request
            .upstream_payload
            .get("input")
            .and_then(|v| v.get("messages"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str())
            .map(|text| format!("proxy mapped: {}", text))
            .unwrap_or_else(|| "proxy mapped: empty".into());
        let bridge_response = BridgeResponse {
            upstream_payload: map_chat_response(model, &assistant_text),
        };

        Ok(bridge_response.upstream_payload)
    }

    pub async fn proxy_response(
        &self,
        model: &str,
        input: &str,
    ) -> Result<Value, String> {
        if !self.check_ready().await {
            return Err("upstream unavailable".into());
        }

        let bridge_request = BridgeRequest {
            upstream_payload: map_response_request(model, input),
        };
        let output_text = bridge_request
            .upstream_payload
            .get("input")
            .and_then(|v| v.as_str())
            .map(|text| format!("proxy mapped: {}", text))
            .unwrap_or_else(|| "proxy mapped: empty".into());
        let bridge_response = BridgeResponse {
            upstream_payload: map_response_output(model, &output_text),
        };

        Ok(bridge_response.upstream_payload)
    }

    pub async fn proxy_codex_app_chat(
        &self,
        model: &str,
        user_text: &str,
        session_namespace: &str,
        session_key_hint: &str,
        freshness_seconds: Option<i64>,
    ) -> Result<Value, String> {
        if !self.check_ready().await {
            return Err("upstream unavailable".into());
        }

        let bridge_request = BridgeRequest {
            upstream_payload: map_codex_app_chat_request(
                model,
                user_text,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            ),
        };
        let assistant_text = bridge_request
            .upstream_payload
            .get("input")
            .and_then(|v| v.get("messages"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str())
            .map(|text| format!("codex-app proxy mapped: {}", text))
            .unwrap_or_else(|| "codex-app proxy mapped: empty".into());
        let bridge_response = BridgeResponse {
            upstream_payload: map_chat_response(model, &assistant_text),
        };

        Ok(bridge_response.upstream_payload)
    }

    pub async fn proxy_codex_app_response(
        &self,
        model: &str,
        input: &str,
        session_namespace: &str,
        session_key_hint: &str,
        freshness_seconds: Option<i64>,
    ) -> Result<Value, String> {
        if !self.check_ready().await {
            return Err("upstream unavailable".into());
        }

        let bridge_request = BridgeRequest {
            upstream_payload: map_codex_app_response_request(
                model,
                input,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            ),
        };
        let output_text = bridge_request
            .upstream_payload
            .get("input")
            .and_then(|v| v.as_str())
            .map(|text| format!("codex-app proxy mapped: {}", text))
            .unwrap_or_else(|| "codex-app proxy mapped: empty".into());
        let bridge_response = BridgeResponse {
            upstream_payload: map_response_output(model, &output_text),
        };

        Ok(bridge_response.upstream_payload)
    }
}
