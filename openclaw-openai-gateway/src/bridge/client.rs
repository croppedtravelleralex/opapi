use serde_json::{json, Value};
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

        Ok(json!({
            "id": format!("chatcmpl-proxy-{}", chrono::Utc::now().timestamp_millis()),
            "object": "chat.completion",
            "created": chrono::Utc::now().timestamp(),
            "model": model,
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": format!("proxy placeholder: {}", user_text)
                },
                "finish_reason": "stop"
            }]
        }))
    }

    pub async fn proxy_response(
        &self,
        model: &str,
        input: &str,
    ) -> Result<Value, String> {
        if !self.check_ready().await {
            return Err("upstream unavailable".into());
        }

        Ok(json!({
            "id": format!("resp-proxy-{}", chrono::Utc::now().timestamp_millis()),
            "object": "response",
            "created_at": chrono::Utc::now().timestamp(),
            "model": model,
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{
                    "type": "output_text",
                    "text": format!("proxy placeholder: {}", input)
                }]
            }]
        }))
    }
}
