use crate::{bridge::client::OpenClawWsClient, config::Config, providers::gateway::GatewayProvider};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub ws_client: Arc<OpenClawWsClient>,
    pub gateway_provider: Arc<GatewayProvider>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, String> {
        let ws_client = Arc::new(OpenClawWsClient::new(
            config.openclaw_ws_url.clone(),
            config.openclaw_api_timeout_ms,
        ));
        let gateway_provider = Arc::new(GatewayProvider::new(ws_client.clone()));

        Ok(Self {
            config,
            ws_client,
            gateway_provider,
        })
    }
}
