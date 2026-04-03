use crate::codex::parser::{parse_quota_page, CodexQuotaPageInput};
use crate::domain::quota_snapshot::QuotaSnapshot;
use rusqlite::{params, Connection};

#[derive(Clone)]
pub struct CodexQuotaCollector {
    dsn: String,
}

impl CodexQuotaCollector {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn collect_from_page_text(&self, input: CodexQuotaPageInput) -> Result<QuotaSnapshot, String> {
        let snapshot = parse_quota_page(&input);
        self.persist_snapshot(&snapshot)?;
        Ok(snapshot)
    }

    fn persist_snapshot(&self, snapshot: &QuotaSnapshot) -> Result<(), String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO quota_snapshots (
                id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
                request_count, token_count, message_count, source_page, confidence, read_ok, error_reason
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                snapshot.id,
                snapshot.child_account_id,
                snapshot.observed_at,
                snapshot.quota_5h_percent,
                snapshot.quota_7d_percent,
                snapshot.request_count,
                snapshot.token_count,
                snapshot.message_count,
                snapshot.source_page,
                snapshot.confidence,
                if snapshot.read_ok { 1_i64 } else { 0_i64 },
                snapshot.error_reason,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
