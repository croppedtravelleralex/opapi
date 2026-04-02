use crate::{error::AppError, state::AppState};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    if path == "/healthz" || path == "/readyz" {
        return Ok(next.run(req).await);
    }

    let auth = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth.strip_prefix("Bearer ").unwrap_or("");
    if state.config.api_keys.iter().any(|k| k == token) {
        Ok(next.run(req).await)
    } else {
        Err(AppError::InvalidApiKey)
    }
}
