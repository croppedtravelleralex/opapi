use axum::{extract::State, Json};
use serde::Serialize;

use crate::app::AppState;

#[derive(Serialize)]
pub struct AccountsResponse {
    pub object: String,
    pub data: Vec<AccountObject>,
}

#[derive(Serialize)]
pub struct AccountObject {
    pub id: i64,
    pub provider: String,
    pub label: String,
    pub status: String,
    pub base_url: String,
    pub model_scope: String,
}

pub async fn list_accounts(State(state): State<AppState>) -> Json<AccountsResponse> {
    let data = state
        .store
        .list_accounts()
        .unwrap_or_default()
        .into_iter()
        .map(|item| AccountObject {
            id: item.id,
            provider: item.provider,
            label: item.label,
            status: item.status,
            base_url: item.base_url,
            model_scope: item.model_scope,
        })
        .collect();

    Json(AccountsResponse {
        object: "list".to_string(),
        data,
    })
}
