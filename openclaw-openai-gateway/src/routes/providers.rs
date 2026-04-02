use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ProviderList {
    pub data: Vec<ProviderItem>,
}

#[derive(Serialize)]
pub struct ProviderItem {
    pub id: String,
    pub class: String,
    pub enabled: bool,
}

pub async fn list_providers(State(state): State<Arc<AppState>>) -> Json<ProviderList> {
    let data = state
        .sqlite_provider_repo
        .list()
        .into_iter()
        .map(|p| ProviderItem {
            id: p.id,
            class: format!("{:?}", p.class),
            enabled: p.enabled,
        })
        .collect();

    Json(ProviderList { data })
}
