use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMember {
    pub id: String,
    pub child_account_id: String,
    pub pool_status: String,
    pub admission_level: String,
    pub weight: i64,
    pub current_load: i64,
    pub cooldown_until: Option<String>,
    pub last_success_at: Option<String>,
    pub last_failure_at: Option<String>,
}
