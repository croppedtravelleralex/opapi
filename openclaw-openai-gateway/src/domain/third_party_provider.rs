use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyProviderImport {
    pub id: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}
