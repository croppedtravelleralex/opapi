use crate::{governance::audit::AuditEvent, repositories::store::InMemoryStore};

#[derive(Clone)]
pub struct AuditRepository {
    store: InMemoryStore,
}

impl AuditRepository {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub fn append(&self, event: &AuditEvent) {
        self.store.audit_events.write().unwrap().push(event.detail.clone());
    }

    pub fn list(&self) -> Vec<String> {
        self.store.audit_events.read().unwrap().clone()
    }
}
