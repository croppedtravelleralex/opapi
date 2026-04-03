use crate::domain::quota_snapshot::QuotaSnapshot;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexQuotaPageInput {
    pub child_account_id: String,
    pub source_id: String,
    pub source_page: String,
    pub page_text: String,
    pub page_html: Option<String>,
    pub snapshot_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCodexQuota {
    pub quota_5h_percent: Option<f64>,
    pub quota_7d_percent: Option<f64>,
    pub request_count: Option<i64>,
    pub token_count: Option<i64>,
    pub message_count: Option<i64>,
    pub confidence: f64,
    pub parser_version: String,
    pub matched_signals: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn parse_quota_page(input: &CodexQuotaPageInput) -> QuotaSnapshot {
    let parsed = extract_quota_fields(&input.page_text);
    let read_ok = parsed.quota_5h_percent.is_some()
        || parsed.quota_7d_percent.is_some()
        || parsed.request_count.is_some()
        || parsed.token_count.is_some()
        || parsed.message_count.is_some();

    QuotaSnapshot {
        id: format!("quota-{}", Uuid::new_v4()),
        child_account_id: input.child_account_id.clone(),
        observed_at: Utc::now().to_rfc3339(),
        quota_5h_percent: parsed.quota_5h_percent,
        quota_7d_percent: parsed.quota_7d_percent,
        request_count: parsed.request_count,
        token_count: parsed.token_count,
        message_count: parsed.message_count,
        source_page: Some(input.source_page.clone()),
        confidence: Some(parsed.confidence),
        read_ok,
        error_reason: if read_ok {
            None
        } else {
            Some("quota_signals_not_found".into())
        },
    }
}

fn extract_quota_fields(text: &str) -> ParsedCodexQuota {
    let quota_5h_percent = extract_percent_after_any(text, &["5h", "5-hour", "5 hour"]);
    let quota_7d_percent = extract_percent_after_any(text, &["7d", "7-day", "7 day"]);
    let request_count = extract_integer_after_any(text, &["requests", "request count"]);
    let token_count = extract_integer_after_any(text, &["tokens", "token count"]);
    let message_count = extract_integer_after_any(text, &["messages", "message count"]);

    let mut matched_signals = Vec::new();
    if quota_5h_percent.is_some() {
        matched_signals.push("quota_5h_percent".into());
    }
    if quota_7d_percent.is_some() {
        matched_signals.push("quota_7d_percent".into());
    }
    if request_count.is_some() {
        matched_signals.push("request_count".into());
    }
    if token_count.is_some() {
        matched_signals.push("token_count".into());
    }
    if message_count.is_some() {
        matched_signals.push("message_count".into());
    }

    let found = matched_signals.len() as f64;
    let confidence = if found == 0.0 { 0.0 } else { (0.35 + found * 0.13).min(0.98) };

    ParsedCodexQuota {
        quota_5h_percent,
        quota_7d_percent,
        request_count,
        token_count,
        message_count,
        confidence,
        parser_version: "codex-parser-v1".into(),
        matched_signals,
        warnings: vec![],
    }
}

fn extract_percent_after_any(text: &str, anchors: &[&str]) -> Option<f64> {
    for anchor in anchors {
        if let Some(value) = extract_percent_after(text, anchor) {
            return Some(value);
        }
    }
    None
}

fn extract_integer_after_any(text: &str, anchors: &[&str]) -> Option<i64> {
    for anchor in anchors {
        if let Some(value) = extract_integer_after(text, anchor) {
            return Some(value);
        }
    }
    None
}

fn extract_percent_after(text: &str, anchor: &str) -> Option<f64> {
    let lower = text.to_lowercase();
    let anchor = anchor.to_lowercase();
    let start = lower.find(&anchor)?;
    let search_start = start + anchor.len();
    let tail = &text[search_start..text.len().min(search_start + 64)];
    let numeric: String = tail
        .chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit() || *ch == '.')
        .collect();
    numeric.parse::<f64>().ok()
}

fn extract_integer_after(text: &str, anchor: &str) -> Option<i64> {
    let lower = text.to_lowercase();
    let anchor = anchor.to_lowercase();
    let start = lower.find(&anchor)?;
    let search_start = start + anchor.len();
    let tail = &text[search_start..text.len().min(search_start + 64)];
    let numeric: String = tail
        .chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    numeric.parse::<i64>().ok()
}
