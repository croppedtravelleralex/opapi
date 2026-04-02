use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::config::Config;

pub async fn require_bearer_auth(
    State(config): State<Config>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    if config.gateway_api_keys.is_empty() {
        return next.run(request).await;
    }

    let auth_header = match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        Some(v) => v,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "message": "missing Authorization header",
                        "type": "authentication_error"
                    }
                })),
            )
                .into_response()
        }
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(v) if !v.trim().is_empty() => v.trim(),
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "message": "invalid Authorization header format",
                        "type": "authentication_error"
                    }
                })),
            )
                .into_response()
        }
    };

    if !config.gateway_api_keys.iter().any(|key| key == token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": {
                    "message": "invalid API key",
                    "type": "authentication_error"
                }
            })),
        )
            .into_response();
    }

    next.run(request).await
}
