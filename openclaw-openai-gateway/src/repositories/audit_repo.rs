use crate::{
    governance::audit::AuditEvent,
    repositories::{sqlite::SqliteAuditRepository, store::InMemoryStore},
};

#[derive(Clone)]
pub struct AuditRepository {
    store: InMemoryStore,
    sqlite: Option<SqliteAuditRepository>,
}

impl AuditRepository {
    pub fn new(store: InMemoryStore) -> Self {
        Self {
            store,
            sqlite: None,
        }
    }

    pub fn with_sqlite(store: InMemoryStore, sqlite: SqliteAuditRepository) -> Self {
        Self {
            store,
            sqlite: Some(sqlite),
        }
    }

    pub fn append(&self, event: &AuditEvent) {
        self.store.audit_events.write().unwrap().push(event.detail.clone());
        if let Some(sqlite) = &self.sqlite {
            let _ = sqlite.append(event);
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.store.audit_events.read().unwrap().clone()
    }
}
