use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderClass {
    Api,
    Gateway,
    Web,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub class: ProviderClass,
    pub enabled: bool,
    pub base_url: Option<String>,
    pub api_key_hint: Option<String>,
}
