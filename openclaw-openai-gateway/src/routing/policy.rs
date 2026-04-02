use crate::domain::routing::{RoutingDecision, RoutingPolicy};

pub fn default_policy() -> RoutingPolicy {
    RoutingPolicy {
        default_provider: "gateway.openclaw".into(),
    }
}

pub fn decide_provider(model: &str, policy: &RoutingPolicy) -> RoutingDecision {
    RoutingDecision {
        model: model.to_string(),
        selected_provider: policy.default_provider.clone(),
        reason: "default gateway provider selected".into(),
    }
}
