use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePlan {
    pub id: String,
    pub title: String,
    pub status: String,
}
