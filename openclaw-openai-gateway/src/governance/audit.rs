use crate::domain::routing::RoutingDecision;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub at: i64,
    pub action: String,
    pub detail: String,
}

pub fn routing_event(decision: &RoutingDecision) -> AuditEvent {
    AuditEvent {
        at: Utc::now().timestamp(),
        action: "routing.decision".into(),
        detail: format!(
            "model={} provider={} reason={}",
            decision.model, decision.selected_provider, decision.reason
        ),
    }
}
