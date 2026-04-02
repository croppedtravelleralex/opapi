use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentAccount {
    pub id: String,
    pub email: String,
    pub space_name: String,
    pub status: String,
    pub fingerprint_profile_id: Option<String>,
    pub invite_enabled: bool,
    pub risk_level: String,
    pub last_login_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildAccount {
    pub id: String,
    pub email: String,
    pub parent_account_id: Option<String>,
    pub status: String,
    pub space_verified: bool,
    pub pool_status: String,
    pub risk_level: String,
    pub fingerprint_profile_id: Option<String>,
    pub last_login_at: Option<String>,
}
