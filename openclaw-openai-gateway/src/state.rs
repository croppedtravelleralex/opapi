use crate::{bridge::client::OpenClawWsClient, config::Config};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub ws_client: Arc<OpenClawWsClient>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, String> {
        let ws_client = Arc::new(OpenClawWsClient::new(
            config.openclaw_ws_url.clone(),
            config.openclaw_api_timeout_ms,
        ));

        Ok(Self { config, ws_client })
    }
}
