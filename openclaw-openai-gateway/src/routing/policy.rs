use crate::domain::routing::{RoutingDecision, RoutingPolicy};
use crate::state::AppState;

pub fn default_policy() -> RoutingPolicy {
    RoutingPolicy {
        default_provider: "gateway.openclaw".into(),
    }
}

pub fn decide_provider(model: &str, policy: &RoutingPolicy, state: Option<&AppState>) -> RoutingDecision {
    let provider = state
        .and_then(|s| s.sqlite_provider_repo.list().into_iter().find(|p| p.enabled))
        .map(|p| p.id)
        .unwrap_or_else(|| policy.default_provider.clone());

    RoutingDecision {
        model: model.to_string(),
        selected_provider: provider,
        reason: "sqlite-backed provider selected".into(),
    }
}
