use crate::domain::routing::RoutingDecision;

pub fn explain(decision: &RoutingDecision) -> String {
    format!(
        "model={} provider={} reason={}",
        decision.model, decision.selected_provider, decision.reason
    )
}
