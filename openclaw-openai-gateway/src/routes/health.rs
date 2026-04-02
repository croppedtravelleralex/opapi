use crate::{error::AppError, state::AppState};
use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    pub status: String,
    pub upstream: String,
    pub providers_total: usize,
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
            providers_total: state.sqlite_provider_repo.list().len(),
        }))
    } else {
        Err(AppError::UpstreamUnavailable)
    }
}
