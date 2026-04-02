use crate::domain::{models::ModelCatalogEntry, provider::ProviderDescriptor};
use crate::repositories::store::InMemoryStore;

#[derive(Clone)]
pub struct SqliteModelRepository {
    pub dsn: String,
    fallback: InMemoryStore,
}

impl SqliteModelRepository {
    pub fn new(dsn: String, fallback: InMemoryStore) -> Self {
        Self { dsn, fallback }
    }

    pub fn list(&self) -> Vec<ModelCatalogEntry> {
        let _ = &self.dsn;
        self.fallback.models.read().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct SqliteProviderRepository {
    pub dsn: String,
    fallback: InMemoryStore,
}

impl SqliteProviderRepository {
    pub fn new(dsn: String, fallback: InMemoryStore) -> Self {
        Self { dsn, fallback }
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        let _ = &self.dsn;
        self.fallback.providers.read().unwrap().clone()
    }
}
