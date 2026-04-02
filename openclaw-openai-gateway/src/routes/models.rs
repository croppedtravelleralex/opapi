use crate::state::AppState;
use axum::{extract::State, Json};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<ModelItem>,
}

#[derive(Serialize)]
pub struct ModelItem {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

pub async fn list_models(State(state): State<Arc<AppState>>) -> Json<ModelList> {
    let now = Utc::now().timestamp();
    let data = state
        .sqlite_model_repo
        .list()
        .iter()
        .map(|entry| ModelItem {
            id: entry.alias.clone().unwrap_or_else(|| entry.canonical_name.clone()),
            object: "model".into(),
            created: now,
            owned_by: entry
                .provider_hint
                .clone()
                .unwrap_or_else(|| "openclaw".into()),
        })
        .collect();

    Json(ModelList {
        object: "list".into(),
        data,
    })
}
