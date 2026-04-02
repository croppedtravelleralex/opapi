use crate::domain::third_party_provider::ThirdPartyProviderImport;

pub fn from_config(config: &crate::config::Config) -> Option<ThirdPartyProviderImport> {
    Some(ThirdPartyProviderImport {
        id: config.third_party_provider_id.clone()?,
        base_url: config.third_party_base_url.clone()?,
        api_key: config.third_party_api_key.clone()?,
        model: config.third_party_model.clone()?,
    })
}
