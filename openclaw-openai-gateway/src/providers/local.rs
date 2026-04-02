use crate::providers::ProviderAdapter;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct LocalProvider;

#[async_trait]
impl ProviderAdapter for LocalProvider {
    async fn check_ready(&self) -> bool {
        true
    }

    async fn chat(&self, model: &str, user_text: &str) -> Result<Value, String> {
        Ok(json!({"provider":"local","model":model,"echo":user_text}))
    }

    async fn response(&self, model: &str, input: &str) -> Result<Value, String> {
        Ok(json!({"provider":"local","model":model,"echo":input}))
    }
}
