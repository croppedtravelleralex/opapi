use crate::domain::routing::{RoutingDecision, RoutingPolicy};
use crate::state::AppState;

pub fn default_policy() -> RoutingPolicy {
    RoutingPolicy {
        default_provider: "gateway.openclaw".into(),
    }
}

pub fn decide_provider(model: &str, policy: &RoutingPolicy, state: Option<&AppState>) -> RoutingDecision {
    if let Some(state) = state {
        let provider = state
            .sqlite_provider_repo
            .list_for_model(model)
            .into_iter()
            .find(|candidate| candidate.availability_status.as_deref() == Some("available"))
            .unwrap_or_else(|| crate::repositories::sqlite::ProviderRoutingRow {
                id: policy.default_provider.clone(),
                availability_status: Some("fallback".into()),
                supports_responses_api: Some(true),
            });

        return RoutingDecision {
            model: model.to_string(),
            selected_provider: provider.id,
            reason: "sqlite-backed capability+availability provider selected".into(),
            availability_status: provider.availability_status,
            supports_responses_api: provider.supports_responses_api,
        };
    }

    RoutingDecision {
        model: model.to_string(),
        selected_provider: policy.default_provider.clone(),
        reason: "default gateway provider selected".into(),
        availability_status: None,
        supports_responses_api: None,
    }
}
