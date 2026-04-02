use crate::providers::ProviderAdapter;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ApiProvider;

#[async_trait]
impl ProviderAdapter for ApiProvider {
    async fn check_ready(&self) -> bool {
        false
    }

    async fn chat(&self, model: &str, user_text: &str) -> Result<Value, String> {
        Ok(json!({"provider":"api","model":model,"echo":user_text}))
    }

    async fn response(&self, model: &str, input: &str) -> Result<Value, String> {
        Ok(json!({"provider":"api","model":model,"echo":input}))
    }
}
