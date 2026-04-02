use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAvailability {
    pub model_name: String,
    pub provider_id: String,
    pub status: String,
}
