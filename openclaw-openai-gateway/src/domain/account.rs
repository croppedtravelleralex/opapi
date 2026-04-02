use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRef {
    pub id: String,
    pub provider_id: String,
    pub enabled: bool,
}
