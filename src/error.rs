use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub fn gateway_error(status: StatusCode, message: impl Into<String>, err_type: &str) -> Response {
    (
        status,
        Json(json!({
            "error": {
                "message": message.into(),
                "type": err_type
            }
        })),
    )
        .into_response()
}
