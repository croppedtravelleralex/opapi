use crate::{error::AppError, providers::ProviderAdapter, state::AppState};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: String,
    pub stream: Option<bool>,
}

#[derive(Serialize)]
pub struct ResponsesResponsePlaceholder {
    pub ignored: bool,
}

pub async fn create_response(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResponsesRequest>,
) -> Result<Json<Value>, AppError> {
    let result = state
        .gateway_provider
        .response(&payload.model, &payload.input)
        .await
        .map_err(|_| AppError::UpstreamUnavailable)?;

    Ok(Json(result))
}
