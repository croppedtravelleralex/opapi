use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyApiKey {
    pub id: String,
    pub label: String,
    pub hashed_key: String,
    pub owner: String,
    pub status: String,
    pub rate_limit: Option<i64>,
    pub quota_limit: Option<i64>,
    pub allowed_models: Vec<String>,
}
