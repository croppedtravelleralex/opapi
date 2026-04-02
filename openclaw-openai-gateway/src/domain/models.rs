use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCatalogEntry {
    pub canonical_name: String,
    pub alias: Option<String>,
    pub provider_hint: Option<String>,
}
