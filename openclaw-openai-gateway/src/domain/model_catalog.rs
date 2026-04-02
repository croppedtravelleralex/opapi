use crate::domain::models::ModelCatalogEntry;

pub fn load_from_config(models: &[String]) -> Vec<ModelCatalogEntry> {
    models
        .iter()
        .map(|name| ModelCatalogEntry {
            canonical_name: name.clone(),
            alias: None,
            provider_hint: Some("gateway.openclaw".into()),
        })
        .collect()
}
