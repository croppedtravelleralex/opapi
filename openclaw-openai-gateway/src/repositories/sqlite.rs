use crate::domain::{
    model_availability::ModelAvailability,
    models::ModelCatalogEntry,
    provider::ProviderDescriptor,
    provider_capability::ProviderCapability,
};
use crate::repositories::store::InMemoryStore;
use rusqlite::{params, Connection};
use serde_json::json;
use std::path::Path;

#[derive(Clone)]
pub struct SqliteModelRepository {
    pub dsn: String,
    fallback: InMemoryStore,
}

impl SqliteModelRepository {
    pub fn new(dsn: String, fallback: InMemoryStore) -> Self {
        Self { dsn, fallback }
    }

    pub fn init_schema(&self) -> Result<(), String> {
        if let Some(parent) = Path::new(&self.dsn).parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS model_catalog (
                id TEXT PRIMARY KEY,
                canonical_name TEXT NOT NULL UNIQUE,
                aliases_json TEXT NOT NULL DEFAULT '[]',
                family TEXT,
                vendor TEXT,
                lifecycle TEXT NOT NULL DEFAULT 'active',
                capability_tags_json TEXT NOT NULL DEFAULT '[]',
                default_route_policy_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS providers (
                id TEXT PRIMARY KEY,
                class TEXT NOT NULL,
                vendor TEXT NOT NULL,
                name TEXT NOT NULL,
                protocol TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                supports_stream INTEGER NOT NULL DEFAULT 0,
                supports_responses_shape INTEGER NOT NULL DEFAULT 0,
                health_score REAL NOT NULL DEFAULT 1.0,
                risk_level TEXT NOT NULL DEFAULT 'normal',
                cost_score REAL NOT NULL DEFAULT 0.0,
                latency_score REAL NOT NULL DEFAULT 0.0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS provider_capabilities (
                id TEXT PRIMARY KEY,
                provider_id TEXT NOT NULL,
                model_name TEXT NOT NULL,
                supports_stream INTEGER NOT NULL DEFAULT 0,
                supports_responses_api INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS model_availability (
                id TEXT PRIMARY KEY,
                model_name TEXT NOT NULL,
                provider_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'available'
            );"
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn seed_models(&self, entries: &[ModelCatalogEntry]) -> Result<(), String> {
        self.init_schema()?;
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        for entry in entries {
            conn.execute(
                "INSERT OR REPLACE INTO model_catalog (
                    id, canonical_name, aliases_json, family, vendor, lifecycle,
                    capability_tags_json, default_route_policy_id, created_at, updated_at
                ) VALUES (?1, ?2, ?3, NULL, NULL, 'active', ?4, NULL, datetime('now'), datetime('now'))",
                params![
                    entry.canonical_name,
                    entry.canonical_name,
                    json!([]).to_string(),
                    json!([]).to_string()
                ],
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<ModelCatalogEntry> {
        if self.init_schema().is_err() {
            return self.fallback.models.read().unwrap().clone();
        }
        let conn = match Connection::open(&self.dsn) {
            Ok(c) => c,
            Err(_) => return self.fallback.models.read().unwrap().clone(),
        };
        let mut stmt = match conn.prepare("SELECT canonical_name FROM model_catalog ORDER BY canonical_name") {
            Ok(s) => s,
            Err(_) => return self.fallback.models.read().unwrap().clone(),
        };
        let rows = stmt
            .query_map([], |row| {
                Ok(ModelCatalogEntry {
                    canonical_name: row.get(0)?,
                    alias: None,
                    provider_hint: Some("gateway.openclaw".into()),
                })
            })
            .ok();
        match rows {
            Some(iter) => iter.filter_map(Result::ok).collect(),
            None => self.fallback.models.read().unwrap().clone(),
        }
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

    pub fn seed_providers(&self, entries: &[ProviderDescriptor]) -> Result<(), String> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), self.fallback.clone());
        model_repo.init_schema()?;
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        for entry in entries {
            conn.execute(
                "INSERT OR REPLACE INTO providers (
                    id, class, vendor, name, protocol, enabled,
                    supports_stream, supports_responses_shape, health_score,
                    risk_level, cost_score, latency_score, created_at, updated_at
                ) VALUES (?1, ?2, 'openclaw', ?3, 'ws', ?4, 0, 1, 1.0, 'normal', 0.0, 0.0, datetime('now'), datetime('now'))",
                params![
                    entry.id,
                    format!("{:?}", entry.class),
                    entry.id,
                    if entry.enabled { 1 } else { 0 }
                ],
            ).map_err(|e| e.to_string())?;

            conn.execute(
                "INSERT OR REPLACE INTO provider_capabilities (
                    id, provider_id, model_name, supports_stream, supports_responses_api
                ) VALUES (?1, ?2, ?3, 0, 1)",
                params![
                    format!("{}::openclaw-default", entry.id),
                    entry.id,
                    "openclaw-default"
                ],
            ).map_err(|e| e.to_string())?;

            conn.execute(
                "INSERT OR REPLACE INTO model_availability (
                    id, model_name, provider_id, status
                ) VALUES (?1, ?2, ?3, 'available')",
                params![
                    format!("openclaw-default::{}", entry.id),
                    "openclaw-default",
                    entry.id
                ],
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), self.fallback.clone());
        if model_repo.init_schema().is_err() {
            return self.fallback.providers.read().unwrap().clone();
        }
        let conn = match Connection::open(&self.dsn) {
            Ok(c) => c,
            Err(_) => return self.fallback.providers.read().unwrap().clone(),
        };
        let mut stmt = match conn.prepare("SELECT id, enabled FROM providers ORDER BY id") {
            Ok(s) => s,
            Err(_) => return self.fallback.providers.read().unwrap().clone(),
        };
        let rows = stmt
            .query_map([], |row| {
                Ok(ProviderDescriptor {
                    id: row.get(0)?,
                    class: crate::domain::provider::ProviderClass::Gateway,
                    enabled: row.get::<_, i64>(1)? != 0,
                })
            })
            .ok();
        match rows {
            Some(iter) => iter.filter_map(Result::ok).collect(),
            None => self.fallback.providers.read().unwrap().clone(),
        }
    }
}
