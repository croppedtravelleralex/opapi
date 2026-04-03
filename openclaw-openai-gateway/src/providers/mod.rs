pub mod gateway;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    async fn check_ready(&self) -> bool;
    async fn chat(&self, model: &str, user_text: &str) -> Result<Value, String>;
    async fn response(&self, model: &str, input: &str) -> Result<Value, String>;
}
