use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

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

pub fn upstream_error(status: StatusCode, body: Value) -> Response {
    (status, Json(body)).into_response()
}

pub fn normalize_upstream_error(status: StatusCode, body: Value) -> Response {
    if body.get("error").is_some() {
        return upstream_error(status, body);
    }

    let message = body
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| body.get("error_message").and_then(Value::as_str))
        .unwrap_or("upstream request failed")
        .to_string();

    let err_type = body
        .get("type")
        .and_then(Value::as_str)
        .or_else(|| body.get("error_type").and_then(Value::as_str))
        .unwrap_or("upstream_error")
        .to_string();

    gateway_error(status, message, &err_type)
}
