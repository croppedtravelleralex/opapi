use crate::{domain::models::ModelCatalogEntry, repositories::store::InMemoryStore};

#[derive(Clone)]
pub struct ModelRepository {
    store: InMemoryStore,
}

impl ModelRepository {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub fn list(&self) -> Vec<ModelCatalogEntry> {
        self.store.models.read().unwrap().clone()
    }

    pub fn replace_all(&self, entries: Vec<ModelCatalogEntry>) {
        *self.store.models.write().unwrap() = entries;
    }
}
