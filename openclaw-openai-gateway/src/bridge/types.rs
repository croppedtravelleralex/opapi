use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BridgeRequest {
    pub upstream_payload: Value,
}

#[derive(Debug, Clone)]
pub struct BridgeResponse {
    pub upstream_payload: Value,
}
