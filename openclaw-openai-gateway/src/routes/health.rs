use crate::{error::AppError, state::AppState};
use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    status: String,
    upstream: String,
}

pub async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}

pub async fn readyz(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ReadyResponse>, AppError> {
    let ok = state.ws_client.check_ready().await;
    if ok {
        Ok(Json(ReadyResponse {
            status: "ready".into(),
            upstream: "ok".into(),
        }))
    } else {
        Err(AppError::UpstreamUnavailable)
    }
}
