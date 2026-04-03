use crate::codex::codex_app_adapter::{CodexAppHandshakeMeta, CodexAppRequestContext};
use crate::repositories::{sqlite::SqliteModelRepository, store::InMemoryStore};
use rusqlite::{params, Connection, OptionalExtension};
use std::env;

#[derive(Clone, Debug)]
pub struct CodexAppSessionSource {
    dsn: String,
    namespace_prefix: String,
    key_salt: String,
}

impl CodexAppSessionSource {
    pub fn from_env(dsn: String) -> Self {
        Self {
            dsn,
            namespace_prefix: env::var("CODEX_APP_SESSION_NAMESPACE_PREFIX")
                .unwrap_or_else(|_| "codex-app".to_string()),
            key_salt: env::var("CODEX_APP_SESSION_KEY_SALT")
                .unwrap_or_else(|_| "local-dev".to_string()),
        }
    }

    pub fn resolve(&self, ctx: &CodexAppRequestContext) -> CodexAppHandshakeMeta {
        let freshness_seconds = chrono::DateTime::parse_from_rfc3339(&ctx.observed_at)
            .ok()
            .map(|dt| chrono::Utc::now().signed_duration_since(dt.with_timezone(&chrono::Utc)).num_seconds())
            .filter(|v| *v >= 0);

        let sqlite = self.resolve_from_sqlite(ctx);
        CodexAppHandshakeMeta {
            session_namespace: sqlite
                .as_ref()
                .and_then(|row| row.session_namespace.clone())
                .unwrap_or_else(|| format!("{}:{}", self.namespace_prefix, ctx.child_account_id)),
            session_key_hint: sqlite
                .as_ref()
                .and_then(|row| row.session_key_hint.clone())
                .unwrap_or_else(|| {
                    format!(
                        "{}:{}:{}:{}",
                        self.key_salt, ctx.child_account_id, ctx.source_id, ctx.source_page
                    )
                }),
            freshness_seconds,
        }
    }

    fn resolve_from_sqlite(&self, ctx: &CodexAppRequestContext) -> Option<CodexAppSessionRow> {
        let model_repo = SqliteModelRepository::new(self.dsn.clone(), InMemoryStore::default());
        model_repo.init_schema().ok()?;
        let conn = Connection::open(&self.dsn).ok()?;
        conn.query_row(
            "SELECT session_namespace, session_key_hint, updated_at
             FROM codex_app_sessions
             WHERE child_account_id = ?1 AND source_id = ?2
             ORDER BY updated_at DESC
             LIMIT 1",
            params![ctx.child_account_id, ctx.source_id],
            |row| {
                Ok(CodexAppSessionRow {
                    session_namespace: row.get(0)?,
                    session_key_hint: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            },
        )
        .optional()
        .ok()
        .flatten()
    }
}

#[derive(Clone, Debug)]
struct CodexAppSessionRow {
    session_namespace: Option<String>,
    session_key_hint: Option<String>,
    #[allow(dead_code)]
    updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn resolve_prefers_sqlite_session_row() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dsn = format!("/tmp/openclaw-codex-app-session-source-{}.sqlite3", unique);
        let repo = SqliteModelRepository::new(dsn.clone(), InMemoryStore::default());
        repo.init_schema().unwrap();
        let conn = Connection::open(&dsn).unwrap();
        conn.execute(
            "INSERT INTO codex_app_sessions (
                id, child_account_id, source_id, session_namespace, session_key_hint, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                "sess-1",
                "child-1",
                "codex-app",
                "sqlite-ns",
                "sqlite-key",
                "2026-04-03T11:00:00+08:00"
            ],
        )
        .unwrap();

        let source = CodexAppSessionSource::from_env(dsn);
        let resolved = source.resolve(&CodexAppRequestContext {
            child_account_id: "child-1".into(),
            source_id: "codex-app".into(),
            source_page: "/codex".into(),
            observed_at: "2026-04-03T11:00:01+08:00".into(),
        });

        assert_eq!(resolved.session_namespace, "sqlite-ns");
        assert_eq!(resolved.session_key_hint, "sqlite-key");
    }
}
