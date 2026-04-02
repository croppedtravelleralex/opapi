use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMembership {
    pub id: String,
    pub parent_account_id: String,
    pub child_account_id: String,
    pub joined: bool,
    pub verified: bool,
    pub verified_at: Option<String>,
}
