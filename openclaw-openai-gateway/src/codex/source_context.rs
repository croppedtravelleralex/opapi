use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct SourceExecutionContext {
    pub child_account_id: String,
    pub source_id: String,
    pub source_page: String,
    pub observed_at: String,
}

#[derive(Clone)]
pub struct SourceContextRepository {
    dsn: String,
}

impl SourceContextRepository {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn latest_for_child(&self, child_account_id: &str) -> Result<SourceExecutionContext, String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        let _ = conn.execute("ALTER TABLE quota_snapshots ADD COLUMN source_id TEXT", []);
        conn.query_row(
            "SELECT child_account_id, COALESCE(source_id, 'unknown'), COALESCE(source_page, '/codex'), observed_at
             FROM quota_snapshots
             WHERE child_account_id = ?1 AND read_ok = 1
             ORDER BY observed_at DESC
             LIMIT 1",
            params![child_account_id],
            |row| {
                Ok(SourceExecutionContext {
                    child_account_id: row.get(0)?,
                    source_id: row.get(1)?,
                    source_page: row.get(2)?,
                    observed_at: row.get(3)?,
                })
            },
        )
        .map_err(|_| format!("missing_source_context_for_child:{}", child_account_id))
    }
}
