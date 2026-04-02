use crate::domain::{models::ModelCatalogEntry, provider::ProviderDescriptor};
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct InMemoryStore {
    pub models: Arc<RwLock<Vec<ModelCatalogEntry>>>,
    pub providers: Arc<RwLock<Vec<ProviderDescriptor>>>,
    pub audit_events: Arc<RwLock<Vec<String>>>,
}
