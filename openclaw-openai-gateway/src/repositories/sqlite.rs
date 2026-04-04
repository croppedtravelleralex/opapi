use crate::domain::{
    models::ModelCatalogEntry,
    provider::ProviderDescriptor,
};
use crate::governance::audit::AuditEvent;
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
                base_url TEXT,
                api_key_hint TEXT,
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
            );
            CREATE TABLE IF NOT EXISTS audit_events (
                id TEXT PRIMARY KEY,
                at INTEGER NOT NULL,
                action TEXT NOT NULL,
                detail TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS parent_accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                space_name TEXT NOT NULL,
                status TEXT NOT NULL,
                fingerprint_profile_id TEXT,
                invite_enabled INTEGER NOT NULL DEFAULT 0,
                risk_level TEXT NOT NULL DEFAULT 'normal',
                last_login_at TEXT
            );
            CREATE TABLE IF NOT EXISTS child_accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                parent_account_id TEXT,
                status TEXT NOT NULL,
                space_verified INTEGER NOT NULL DEFAULT 0,
                pool_status TEXT NOT NULL DEFAULT 'new',
                risk_level TEXT NOT NULL DEFAULT 'normal',
                fingerprint_profile_id TEXT,
                last_login_at TEXT
            );
            CREATE TABLE IF NOT EXISTS space_memberships (
                id TEXT PRIMARY KEY,
                parent_account_id TEXT NOT NULL,
                child_account_id TEXT NOT NULL UNIQUE,
                joined INTEGER NOT NULL DEFAULT 0,
                verified INTEGER NOT NULL DEFAULT 0,
                verified_at TEXT
            );
            CREATE TABLE IF NOT EXISTS invite_tasks (
                id TEXT PRIMARY KEY,
                parent_account_id TEXT NOT NULL,
                child_account_id TEXT NOT NULL,
                status TEXT NOT NULL,
                sent_at TEXT,
                accepted_at TEXT,
                error_reason TEXT
            );
            CREATE TABLE IF NOT EXISTS registration_tasks (
                id TEXT PRIMARY KEY,
                parent_account_id TEXT NOT NULL,
                child_account_id TEXT NOT NULL,
                task_type TEXT NOT NULL,
                status TEXT NOT NULL,
                payload_json TEXT NOT NULL DEFAULT '{}',
                result_json TEXT,
                queued_at TEXT NOT NULL,
                started_at TEXT,
                finished_at TEXT,
                error_reason TEXT,
                attempt_count INTEGER NOT NULL DEFAULT 0,
                max_attempts INTEGER NOT NULL DEFAULT 3,
                next_run_at TEXT,
                lease_until TEXT,
                risk_score INTEGER NOT NULL DEFAULT 0,
                dead_lettered INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS verification_tasks (
                id TEXT PRIMARY KEY,
                child_account_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                status TEXT NOT NULL,
                provider TEXT,
                verification_target TEXT,
                code_hint TEXT,
                last_checked_at TEXT,
                verified_at TEXT,
                error_reason TEXT
            );
            CREATE TABLE IF NOT EXISTS quota_snapshots (
                id TEXT PRIMARY KEY,
                child_account_id TEXT NOT NULL,
                observed_at TEXT NOT NULL,
                quota_5h_percent REAL,
                quota_7d_percent REAL,
                request_count INTEGER,
                token_count INTEGER,
                message_count INTEGER,
                source_id TEXT,
                source_page TEXT,
                confidence REAL,
                read_ok INTEGER NOT NULL DEFAULT 1,
                error_reason TEXT
            );
            CREATE TABLE IF NOT EXISTS pool_members (
                id TEXT PRIMARY KEY,
                child_account_id TEXT NOT NULL UNIQUE,
                pool_status TEXT NOT NULL,
                admission_level TEXT NOT NULL,
                weight INTEGER NOT NULL DEFAULT 1,
                current_load INTEGER NOT NULL DEFAULT 0,
                cooldown_until TEXT,
                last_success_at TEXT,
                last_failure_at TEXT
            );
            CREATE TABLE IF NOT EXISTS codex_app_sessions (
                id TEXT PRIMARY KEY,
                child_account_id TEXT NOT NULL,
                source_id TEXT NOT NULL DEFAULT 'codex-app',
                session_namespace TEXT,
                session_key_hint TEXT,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS proxy_api_keys (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                hashed_key TEXT NOT NULL,
                owner TEXT NOT NULL,
                status TEXT NOT NULL,
                rate_limit INTEGER,
                quota_limit INTEGER,
                allowed_models_json TEXT NOT NULL DEFAULT '[]'
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
pub struct SqliteAuditRepository {
    pub dsn: String,
}

impl SqliteAuditRepository {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn append(&self, event: &AuditEvent) -> Result<(), String> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), InMemoryStore::default());
        model_repo.init_schema()?;
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO audit_events (id, at, action, detail) VALUES (?1, ?2, ?3, ?4)",
            params![
                format!("audit-{}-{}", event.action, event.at),
                event.at,
                event.action,
                event.detail
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn count(&self) -> Result<i64, String> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), InMemoryStore::default());
        model_repo.init_schema()?;
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.query_row("SELECT COUNT(*) FROM audit_events", [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ProviderRoutingRow {
    pub id: String,
    pub availability_status: Option<String>,
    pub supports_responses_api: Option<bool>,
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
            let protocol = if entry.base_url.is_some() { "https" } else { "ws" };
            conn.execute(
                "INSERT OR REPLACE INTO providers (
                    id, class, vendor, name, protocol, enabled, base_url, api_key_hint,
                    supports_stream, supports_responses_shape, health_score,
                    risk_level, cost_score, latency_score, created_at, updated_at
                ) VALUES (?1, ?2, 'openclaw', ?3, ?4, ?5, ?6, ?7, 0, 1, 1.0, 'normal', 0.0, 0.0, datetime('now'), datetime('now'))",
                params![
                    entry.id,
                    format!("{:?}", entry.class),
                    entry.id,
                    protocol,
                    if entry.enabled { 1 } else { 0 },
                    entry.base_url,
                    entry.api_key_hint
                ],
            ).map_err(|e| e.to_string())?;

            let model_name = if entry.id == "api.openai-compatible-demo" {
                "gpt-4o-mini"
            } else {
                "openclaw-default"
            };

            conn.execute(
                "INSERT OR REPLACE INTO provider_capabilities (
                    id, provider_id, model_name, supports_stream, supports_responses_api
                ) VALUES (?1, ?2, ?3, 0, 1)",
                params![
                    format!("{}::{}", entry.id, model_name),
                    entry.id,
                    model_name
                ],
            ).map_err(|e| e.to_string())?;

            conn.execute(
                "INSERT OR REPLACE INTO model_availability (
                    id, model_name, provider_id, status
                ) VALUES (?1, ?2, ?3, 'available')",
                params![
                    format!("{}::{}", model_name, entry.id),
                    model_name,
                    entry.id
                ],
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn list_for_model(&self, model: &str) -> Vec<ProviderRoutingRow> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), self.fallback.clone());
        if model_repo.init_schema().is_err() {
            return vec![];
        }
        let conn = match Connection::open(&self.dsn) {
            Ok(c) => c,
            Err(_) => return vec![],
        };
        let mut stmt = match conn.prepare(
            "SELECT p.id, ma.status, pc.supports_responses_api
             FROM providers p
             LEFT JOIN model_availability ma ON ma.provider_id = p.id AND ma.model_name = ?1
             LEFT JOIN provider_capabilities pc ON pc.provider_id = p.id AND pc.model_name = ?1
             WHERE p.enabled = 1
             ORDER BY p.id"
        ) {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let rows = stmt
            .query_map([model], |row| {
                Ok(ProviderRoutingRow {
                    id: row.get(0)?,
                    availability_status: row.get(1).ok(),
                    supports_responses_api: row.get::<_, Option<i64>>(2)?.map(|v| v != 0),
                })
            })
            .ok();
        match rows {
            Some(iter) => iter.filter_map(Result::ok).collect(),
            None => vec![],
        }
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
        let mut stmt = match conn.prepare("SELECT id, class, enabled, base_url, api_key_hint FROM providers ORDER BY id") {
            Ok(s) => s,
            Err(_) => return self.fallback.providers.read().unwrap().clone(),
        };
        let rows = stmt
            .query_map([], |row| {
                let class_str: String = row.get(1)?;
                let class = match class_str.as_str() {
                    "Api" => crate::domain::provider::ProviderClass::Api,
                    "Web" => crate::domain::provider::ProviderClass::Web,
                    "Local" => crate::domain::provider::ProviderClass::Local,
                    _ => crate::domain::provider::ProviderClass::Gateway,
                };
                Ok(ProviderDescriptor {
                    id: row.get(0)?,
                    class,
                    enabled: row.get::<_, i64>(2)? != 0,
                    base_url: row.get(3).ok(),
                    api_key_hint: row.get(4).ok(),
                })
            })
            .ok();
        match rows {
            Some(iter) => iter.filter_map(Result::ok).collect(),
            None => self.fallback.providers.read().unwrap().clone(),
        }
    }
}
