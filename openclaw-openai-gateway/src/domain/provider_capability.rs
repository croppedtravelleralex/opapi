use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    pub provider_id: String,
    pub model_name: String,
    pub supports_stream: bool,
    pub supports_responses_api: bool,
}
