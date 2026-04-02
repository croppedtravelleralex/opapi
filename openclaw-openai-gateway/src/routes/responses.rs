use crate::{
    error::AppError,
    observability::explain::explain,
    providers::ProviderAdapter,
    routing::policy::{decide_provider, default_policy},
    state::AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: String,
    pub stream: Option<bool>,
}

pub async fn create_response(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResponsesRequest>,
) -> Result<impl IntoResponse, AppError> {
    let decision = decide_provider(&payload.model, &default_policy());
    let explain_text = explain(&decision);

    let result = state
        .gateway_provider
        .response(&payload.model, &payload.input)
        .await
        .map_err(|_| AppError::UpstreamUnavailable)?;

    Ok(([ ("x-routing-explain", explain_text) ], Json(result)))
}
