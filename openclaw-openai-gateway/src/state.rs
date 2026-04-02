use crate::{
    bridge::client::OpenClawWsClient,
    config::Config,
    domain::{
        model_catalog::load_from_config,
        models::ModelCatalogEntry,
        provider::{ProviderClass, ProviderDescriptor},
        provider_pool::default_provider_pool,
    },
    providers::gateway::GatewayProvider,
    repositories::{
        audit_repo::AuditRepository,
        model_repo::ModelRepository,
        provider_repo::ProviderRepository,
        sqlite::{SqliteAuditRepository, SqliteModelRepository, SqliteProviderRepository},
        store::InMemoryStore,
    },
};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub ws_client: Arc<OpenClawWsClient>,
    pub gateway_provider: Arc<GatewayProvider>,
    pub model_catalog: Vec<ModelCatalogEntry>,
    pub provider_pool: Vec<ProviderDescriptor>,
    pub store: InMemoryStore,
    pub model_repo: ModelRepository,
    pub provider_repo: ProviderRepository,
    pub audit_repo: AuditRepository,
    pub sqlite_model_repo: SqliteModelRepository,
    pub sqlite_provider_repo: SqliteProviderRepository,
    pub sqlite_audit_repo: SqliteAuditRepository,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, String> {
        let ws_client = Arc::new(OpenClawWsClient::new(
            config.openclaw_ws_url.clone(),
            config.openclaw_api_timeout_ms,
        ));
        let gateway_provider = Arc::new(GatewayProvider::new(ws_client.clone()));
        let mut model_catalog = load_from_config(&config.models);
        let mut provider_pool = default_provider_pool();

        if let Some(imported) = crate::providers::import::from_config(&config) {
            provider_pool.push(ProviderDescriptor {
                id: imported.id.clone(),
                class: ProviderClass::Api,
                enabled: true,
                base_url: Some(imported.base_url.clone()),
                api_key_hint: Some(mask_api_key(&imported.api_key)),
            });
            model_catalog.push(ModelCatalogEntry {
                canonical_name: imported.model.clone(),
                alias: None,
                provider_hint: Some(imported.id.clone()),
            });
        }

        let store = InMemoryStore::default();
        let model_repo = ModelRepository::new(store.clone());
        let provider_repo = ProviderRepository::new(store.clone());
        let sqlite_audit_repo = SqliteAuditRepository::new(config.sqlite_path.clone());
        let audit_repo = AuditRepository::with_sqlite(store.clone(), sqlite_audit_repo.clone());
        model_repo.replace_all(model_catalog.clone());
        provider_repo.replace_all(provider_pool.clone());

        let sqlite_model_repo = SqliteModelRepository::new(config.sqlite_path.clone(), store.clone());
        let sqlite_provider_repo = SqliteProviderRepository::new(config.sqlite_path.clone(), store.clone());
        let _ = sqlite_model_repo.seed_models(&model_catalog);
        let _ = sqlite_provider_repo.seed_providers(&provider_pool);

        Ok(Self {
            config,
            ws_client,
            gateway_provider,
            model_catalog,
            provider_pool,
            store,
            model_repo,
            provider_repo,
            audit_repo,
            sqlite_model_repo,
            sqlite_provider_repo,
            sqlite_audit_repo,
        })
    }
}

fn mask_api_key(raw: &str) -> String {
    if raw.len() <= 6 {
        return "***".into();
    }
    format!("{}***", &raw[..6])
}
