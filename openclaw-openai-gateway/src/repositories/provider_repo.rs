use crate::{domain::provider::ProviderDescriptor, repositories::store::InMemoryStore};

#[derive(Clone)]
pub struct ProviderRepository {
    store: InMemoryStore,
}

impl ProviderRepository {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        self.store.providers.read().unwrap().clone()
    }

    pub fn replace_all(&self, entries: Vec<ProviderDescriptor>) {
        *self.store.providers.write().unwrap() = entries;
    }
}
