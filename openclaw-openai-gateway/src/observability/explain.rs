use crate::domain::routing::RoutingDecision;

pub fn explain(decision: &RoutingDecision) -> String {
    format!(
        "model={} provider={} reason={} availability_status={} supports_responses_api={}",
        decision.model,
        decision.selected_provider,
        decision.reason,
        decision.availability_status.clone().unwrap_or_else(|| "unknown".into()),
        decision
            .supports_responses_api
            .map(|v| v.to_string())
            .unwrap_or_else(|| "unknown".into())
    )
}
