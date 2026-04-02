use crate::providers::ProviderAdapter;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ApiProvider {
    pub base_url: String,
    pub api_key_hint: String,
}

#[async_trait]
impl ProviderAdapter for ApiProvider {
    async fn check_ready(&self) -> bool {
        !self.base_url.is_empty() && !self.api_key_hint.is_empty()
    }

    async fn chat(&self, model: &str, user_text: &str) -> Result<Value, String> {
        Ok(json!({
            "provider":"api",
            "base_url": self.base_url,
            "api_key_hint": self.api_key_hint,
            "model":model,
            "echo":user_text
        }))
    }

    async fn response(&self, model: &str, input: &str) -> Result<Value, String> {
        Ok(json!({
            "provider":"api",
            "base_url": self.base_url,
            "api_key_hint": self.api_key_hint,
            "model":model,
            "echo":input
        }))
    }
}
