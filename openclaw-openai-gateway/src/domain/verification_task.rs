use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTask {
    pub id: String,
    pub child_account_id: String,
    pub kind: String,
    pub status: String,
    pub provider: Option<String>,
    pub verification_target: Option<String>,
    pub code_hint: Option<String>,
    pub last_checked_at: Option<String>,
    pub verified_at: Option<String>,
    pub error_reason: Option<String>,
}
