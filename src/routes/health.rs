use axum::{extract::State, Json};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub service: String,
}

pub async fn healthz(State(config): State<Config>) -> Json<HealthResponse> {
    Json(HealthResponse {
        ok: true,
        service: config.api_title.clone(),
    })
}

pub async fn readyz(State(config): State<Config>) -> Json<HealthResponse> {
    Json(HealthResponse {
        ok: true,
        service: config.api_title.clone(),
    })
}
