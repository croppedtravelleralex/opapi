use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaSnapshot {
    pub id: String,
    pub child_account_id: String,
    pub observed_at: String,
    pub quota_5h_percent: Option<f64>,
    pub quota_7d_percent: Option<f64>,
    pub request_count: Option<i64>,
    pub token_count: Option<i64>,
    pub message_count: Option<i64>,
    pub source_page: Option<String>,
    pub confidence: Option<f64>,
    pub read_ok: bool,
    pub error_reason: Option<String>,
}
