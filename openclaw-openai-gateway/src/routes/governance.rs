use crate::state::AppState;
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn get_config_snapshot(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let snapshot = crate::governance::config_snapshot::capture(
        "sqlite-backed control-plane snapshot",
        "runtime",
    );
    Json(serde_json::json!(snapshot))
}

pub async fn get_release_record(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let record = crate::governance::release_record::draft("sqlite-backed provider rollout");
    Json(serde_json::json!(record))
}

pub async fn get_change_plan(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let plan = crate::governance::change_plan::draft("promote sqlite control-plane reads");
    Json(serde_json::json!(plan))
}
