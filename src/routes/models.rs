use axum::{extract::State, Json};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub owned_by: String,
    pub upstream: String,
    pub endpoint: String,
}

#[derive(Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

pub async fn list_models(State(config): State<Config>) -> Json<ModelsResponse> {
    let data = config
        .default_models
        .iter()
        .map(|model| {
            let (upstream, endpoint) = match config.upstream_for_model(model) {
                Some(upstream) => {
                    let endpoint = if upstream.append_v1 {
                        format!("{}/v1/chat/completions", upstream.base_url)
                    } else {
                        format!("{}/chat/completions", upstream.base_url)
                    };
                    (upstream.name, endpoint)
                }
                None => ("unconfigured".to_string(), "unconfigured".to_string()),
            };

            ModelObject {
                id: model.clone(),
                object: "model".to_string(),
                owned_by: "sub2api-gateway".to_string(),
                upstream,
                endpoint,
            }
        })
        .collect();

    Json(ModelsResponse {
        object: "list".to_string(),
        data,
    })
}
