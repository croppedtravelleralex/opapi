use crate::bridge::{
    mock::{build_codex_app_chat_payload, build_codex_app_response_payload},
    real::{proxy_codex_app_chat as real_codex_app_chat, proxy_codex_app_response as real_codex_app_response},
    mapper::{map_chat_request, map_chat_response, map_response_output, map_response_request},
    types::{BridgeRequest, BridgeResponse},
};
use serde_json::Value;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenClawWsTransportMode {
    Mock,
    Real,
}

impl OpenClawWsTransportMode {
    pub fn from_str(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "real" => Self::Real,
            _ => Self::Mock,
        }
    }
}

pub struct OpenClawWsClient {
    url: String,
    timeout_ms: u64,
    mode: OpenClawWsTransportMode,
}

impl OpenClawWsClient {
    pub fn new(url: String, timeout_ms: u64, mode: OpenClawWsTransportMode) -> Self {
        Self { url, timeout_ms, mode }
    }

    pub async fn check_ready(&self) -> bool {
        let connect = timeout(Duration::from_millis(self.timeout_ms), connect_async(&self.url)).await;
        matches!(connect, Ok(Ok((_ws, _resp))))
    }

    pub async fn proxy_chat(&self, model: &str, user_text: &str) -> Result<Value, String> {
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

    pub async fn proxy_response(&self, model: &str, input: &str) -> Result<Value, String> {
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

        match self.mode {
            OpenClawWsTransportMode::Mock => Ok(build_codex_app_chat_payload(
                model,
                user_text,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            )
            .await),
            OpenClawWsTransportMode::Real => real_codex_app_chat(
                &self.url,
                self.timeout_ms,
                model,
                user_text,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            )
            .await,
        }
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

        match self.mode {
            OpenClawWsTransportMode::Mock => Ok(build_codex_app_response_payload(
                model,
                input,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            )
            .await),
            OpenClawWsTransportMode::Real => real_codex_app_response(
                &self.url,
                self.timeout_ms,
                model,
                input,
                session_namespace,
                session_key_hint,
                freshness_seconds,
            )
            .await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_defaults_to_mock_and_accepts_real() {
        assert_eq!(OpenClawWsTransportMode::from_str("real"), OpenClawWsTransportMode::Real);
        assert_eq!(OpenClawWsTransportMode::from_str("REAL"), OpenClawWsTransportMode::Real);
        assert_eq!(OpenClawWsTransportMode::from_str(" mock "), OpenClawWsTransportMode::Mock);
        assert_eq!(OpenClawWsTransportMode::from_str("anything-else"), OpenClawWsTransportMode::Mock);
    }

    #[test]
    fn new_keeps_transport_mode() {
        let client = OpenClawWsClient::new(
            "ws://127.0.0.1:1".into(),
            25,
            OpenClawWsTransportMode::Real,
        );
        assert_eq!(client.mode, OpenClawWsTransportMode::Real);
    }
}
