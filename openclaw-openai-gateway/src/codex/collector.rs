use crate::codex::parser::{parse_quota_page, CodexQuotaPageInput};
use crate::domain::quota_snapshot::QuotaSnapshot;
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct CodexAppSessionInput {
    pub session_namespace: Option<String>,
    pub session_key_hint: Option<String>,
}

#[derive(Clone)]
pub struct CodexQuotaCollector {
    dsn: String,
}

impl CodexQuotaCollector {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn collect_from_page_text(
        &self,
        input: CodexQuotaPageInput,
        session: CodexAppSessionInput,
    ) -> Result<QuotaSnapshot, String> {
        let snapshot = parse_quota_page(&input);
        let resolved_session = merge_session_input(&input, &session);
        self.persist_snapshot(&snapshot)?;
        self.persist_codex_app_session(&input, &resolved_session)?;
        Ok(snapshot)
    }

    fn persist_snapshot(&self, snapshot: &QuotaSnapshot) -> Result<(), String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        let _ = conn.execute("ALTER TABLE quota_snapshots ADD COLUMN source_id TEXT", []);
        conn.execute(
            "INSERT INTO quota_snapshots (
                id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
                request_count, token_count, message_count, source_id, source_page, confidence, read_ok, error_reason
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                snapshot.id,
                snapshot.child_account_id,
                snapshot.observed_at,
                snapshot.quota_5h_percent,
                snapshot.quota_7d_percent,
                snapshot.request_count,
                snapshot.token_count,
                snapshot.message_count,
                snapshot.source_id,
                snapshot.source_page,
                snapshot.confidence,
                if snapshot.read_ok { 1_i64 } else { 0_i64 },
                snapshot.error_reason,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn persist_codex_app_session(
        &self,
        input: &CodexQuotaPageInput,
        session: &CodexAppSessionInput,
    ) -> Result<(), String> {
        if input.source_id != "codex-app" {
            return Ok(());
        }
        if session.session_namespace.is_none() && session.session_key_hint.is_none() {
            return Ok(());
        }

        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO codex_app_sessions (
                id, child_account_id, source_id, session_namespace, session_key_hint, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
            params![
                format!("codex-app-session:{}:{}", input.child_account_id, input.source_id),
                input.child_account_id,
                input.source_id,
                session.session_namespace,
                session.session_key_hint,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn merge_session_input(input: &CodexQuotaPageInput, session: &CodexAppSessionInput) -> CodexAppSessionInput {
    let extracted = extract_session_input(input);
    CodexAppSessionInput {
        session_namespace: session
            .session_namespace
            .clone()
            .or(extracted.session_namespace),
        session_key_hint: session
            .session_key_hint
            .clone()
            .or(extracted.session_key_hint),
    }
}

fn extract_session_input(input: &CodexQuotaPageInput) -> CodexAppSessionInput {
    CodexAppSessionInput {
        session_namespace: extract_session_value(
            &input.page_text,
            input.page_html.as_deref(),
            &["session namespace", "session_namespace", "session-namespace"],
        ),
        session_key_hint: extract_session_value(
            &input.page_text,
            input.page_html.as_deref(),
            &["session key hint", "session_key_hint", "session-key-hint"],
        ),
    }
}

fn extract_session_value(text: &str, html: Option<&str>, anchors: &[&str]) -> Option<String> {
    for anchor in anchors {
        if let Some(value) = extract_value_after_anchor(text, anchor) {
            return Some(value);
        }
        if let Some(raw_html) = html {
            if let Some(value) = extract_value_after_anchor(raw_html, anchor) {
                return Some(value);
            }
        }
    }
    None
}

fn extract_value_after_anchor(content: &str, anchor: &str) -> Option<String> {
    let lower = content.to_lowercase();
    let anchor_lower = anchor.to_lowercase();
    let start = lower.find(&anchor_lower)?;
    let tail = &content[start + anchor.len()..content.len().min(start + anchor.len() + 160)];
    let cleaned = tail
        .trim_start_matches(|ch: char| ch.is_whitespace() || matches!(ch, ':' | '=' | '"' | '\'' | '>'));
    let value: String = cleaned
        .chars()
        .take_while(|ch| !ch.is_whitespace() && !matches!(ch, '<' | '"' | '\'' | ',' | ';'))
        .collect();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
