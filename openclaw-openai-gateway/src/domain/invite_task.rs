use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteTask {
    pub id: String,
    pub parent_account_id: String,
    pub child_account_id: String,
    pub status: String,
    pub sent_at: Option<String>,
    pub accepted_at: Option<String>,
    pub error_reason: Option<String>,
}
