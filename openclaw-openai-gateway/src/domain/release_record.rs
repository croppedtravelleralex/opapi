use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseRecord {
    pub id: String,
    pub version: String,
    pub note: String,
}
