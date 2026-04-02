use crate::{
    bridge::client::OpenClawWsClient,
    config::Config,
    domain::{
        model_catalog::load_from_config,
        models::ModelCatalogEntry,
        provider::ProviderDescriptor,
        provider_pool::default_provider_pool,
    },
    providers::gateway::GatewayProvider,
};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub ws_client: Arc<OpenClawWsClient>,
    pub gateway_provider: Arc<GatewayProvider>,
    pub model_catalog: Vec<ModelCatalogEntry>,
    pub provider_pool: Vec<ProviderDescriptor>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, String> {
        let ws_client = Arc::new(OpenClawWsClient::new(
            config.openclaw_ws_url.clone(),
            config.openclaw_api_timeout_ms,
        ));
        let gateway_provider = Arc::new(GatewayProvider::new(ws_client.clone()));
        let model_catalog = load_from_config(&config.models);
        let provider_pool = default_provider_pool();

        Ok(Self {
            config,
            ws_client,
            gateway_provider,
            model_catalog,
            provider_pool,
        })
    }
}
