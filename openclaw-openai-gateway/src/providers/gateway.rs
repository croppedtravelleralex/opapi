use crate::{bridge::client::OpenClawWsClient, providers::ProviderAdapter};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

pub struct GatewayProvider {
    #[allow(dead_code)]
    client: Arc<OpenClawWsClient>,
}

impl GatewayProvider {
    pub fn new(client: Arc<OpenClawWsClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProviderAdapter for GatewayProvider {
    async fn check_ready(&self) -> bool {
        self.client.check_ready().await
    }

    async fn chat(&self, model: &str, user_text: &str) -> Result<Value, String> {
        self.client.proxy_chat(model, user_text).await
    }

    async fn response(&self, model: &str, input: &str) -> Result<Value, String> {
        self.client.proxy_response(model, input).await
    }
}
