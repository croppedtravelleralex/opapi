use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{app::AppState, store::ImportedAccount};

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
    pub email: String,
    pub user_id: String,
    pub region: String,
    pub trial_end_time: i64,
    pub cashier_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportedAccountPayload {
    pub platform: String,
    pub email: String,
    pub password: String,
    pub user_id: Option<String>,
    pub region: Option<String>,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub status: Option<String>,
    pub trial_end_time: Option<i64>,
    pub cashier_url: Option<String>,
    pub extra_json: Option<String>,
    pub provider: Option<String>,
    pub label: Option<String>,
    pub base_url: Option<String>,
    pub model_scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportedAccountsBatch {
    pub accounts: Vec<ImportedAccountPayload>,
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
            email: item.email,
            user_id: item.user_id,
            region: item.region,
            trial_end_time: item.trial_end_time,
            cashier_url: item.cashier_url,
        })
        .collect();

    Json(AccountsResponse {
        object: "list".to_string(),
        data,
    })
}

pub async fn import_accounts(
    State(state): State<AppState>,
    Json(payload): Json<ImportedAccountsBatch>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut imported = 0usize;

    for item in payload.accounts {
        let provider = item.provider.unwrap_or_else(|| item.platform.clone());
        let label = item
            .label
            .unwrap_or_else(|| format!("{}:{}", provider, item.email));

        let account = ImportedAccount {
            provider,
            label,
            status: item.status.unwrap_or_else(|| "registered".to_string()),
            base_url: item.base_url.unwrap_or_default(),
            model_scope: item.model_scope.unwrap_or_default(),
            email: item.email,
            password: item.password,
            user_id: item.user_id.unwrap_or_default(),
            region: item.region.unwrap_or_default(),
            token: item.token.unwrap_or_default(),
            refresh_token: item.refresh_token.unwrap_or_default(),
            trial_end_time: item.trial_end_time.unwrap_or(0),
            cashier_url: item.cashier_url.unwrap_or_default(),
            extra_json: item.extra_json.unwrap_or_else(|| "{}".to_string()),
        };

        if state.store.upsert_imported_account(account).is_ok() {
            imported += 1;
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "object": "import_result",
            "imported": imported
        })),
    )
}

pub async fn update_account_status(
    State(state): State<AppState>,
    Json(payload): Json<ImportedAccountPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    let provider = payload.provider.unwrap_or_else(|| payload.platform.clone());
    let label = payload
        .label
        .unwrap_or_else(|| format!("{}:{}", provider, payload.email));

    let account = ImportedAccount {
        provider,
        label,
        status: payload.status.unwrap_or_else(|| "registered".to_string()),
        base_url: payload.base_url.unwrap_or_default(),
        model_scope: payload.model_scope.unwrap_or_default(),
        email: payload.email,
        password: payload.password,
        user_id: payload.user_id.unwrap_or_default(),
        region: payload.region.unwrap_or_default(),
        token: payload.token.unwrap_or_default(),
        refresh_token: payload.refresh_token.unwrap_or_default(),
        trial_end_time: payload.trial_end_time.unwrap_or(0),
        cashier_url: payload.cashier_url.unwrap_or_default(),
        extra_json: payload.extra_json.unwrap_or_else(|| "{}".to_string()),
    };

    let ok = state.store.upsert_imported_account(account).is_ok();

    (
        if ok { StatusCode::OK } else { StatusCode::BAD_GATEWAY },
        Json(json!({
            "object": "status_update_result",
            "ok": ok
        })),
    )
}
