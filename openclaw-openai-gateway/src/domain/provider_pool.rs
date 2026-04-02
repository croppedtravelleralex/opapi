use crate::domain::provider::{ProviderClass, ProviderDescriptor};

pub fn default_provider_pool() -> Vec<ProviderDescriptor> {
    vec![ProviderDescriptor {
        id: "gateway.openclaw".into(),
        class: ProviderClass::Gateway,
        enabled: true,
        base_url: None,
        api_key_hint: None,
    }]
}
