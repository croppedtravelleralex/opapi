use axum::{extract::State, Json};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub owned_by: String,
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
        .map(|model| ModelObject {
            id: model.clone(),
            object: "model".to_string(),
            owned_by: "sub2api-gateway".to_string(),
        })
        .collect();

    Json(ModelsResponse {
        object: "list".to_string(),
        data,
    })
}
