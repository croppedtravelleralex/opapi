use crate::state::AppState;
use axum::{extract::State, Json};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct ModelList {
    object: String,
    data: Vec<ModelItem>,
}

#[derive(Serialize)]
struct ModelItem {
    id: String,
    object: String,
    created: i64,
    owned_by: String,
}

pub async fn list_models(State(state): State<Arc<AppState>>) -> Json<ModelList> {
    let now = Utc::now().timestamp();
    let data = state
        .config
        .models
        .iter()
        .map(|id| ModelItem {
            id: id.clone(),
            object: "model".into(),
            created: now,
            owned_by: "openclaw".into(),
        })
        .collect();

    Json(ModelList {
        object: "list".into(),
        data,
    })
}
