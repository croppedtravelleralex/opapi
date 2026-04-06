use axum::{extract::State, Json};
use serde::Serialize;

use crate::{app::AppState, config::UpstreamConfig};

#[derive(Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub owned_by: String,
    pub upstream: String,
    pub endpoint: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

pub async fn list_models(State(state): State<AppState>) -> Json<ModelsResponse> {
    let config = &state.config;
    let accounts = state.store.list_accounts().unwrap_or_default();

    let data = config
        .default_models
        .iter()
        .map(|model| {
            let upstream = config.upstream_for_model(model);
            let (upstream_name, endpoint, status) = match upstream {
                Some(ref upstream) => describe_upstream(upstream, &accounts, model),
                None => (
                    "unconfigured".to_string(),
                    "unconfigured".to_string(),
                    "unconfigured".to_string(),
                ),
            };

            ModelObject {
                id: model.clone(),
                object: "model".to_string(),
                owned_by: "sub2api-gateway".to_string(),
                upstream: upstream_name,
                endpoint,
                status,
            }
        })
        .collect();

    Json(ModelsResponse {
        object: "list".to_string(),
        data,
    })
}

fn describe_upstream(
    upstream: &UpstreamConfig,
    accounts: &[crate::store::AccountRecord],
    model: &str,
) -> (String, String, String) {
    let endpoint = if upstream.append_v1 {
        format!("{}/v1/chat/completions", upstream.base_url)
    } else {
        format!("{}/chat/completions", upstream.base_url)
    };

    let status = accounts
        .iter()
        .find(|item| item.provider == upstream.name && item.model_scope == model)
        .map(|item| item.status.clone())
        .unwrap_or_else(|| "configured".to_string());

    (upstream.name.clone(), endpoint, status)
}
