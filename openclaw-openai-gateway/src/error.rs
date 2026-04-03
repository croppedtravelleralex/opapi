use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid api key")]
    InvalidApiKey,
    #[error("upstream unavailable")]
    UpstreamUnavailable,
    #[error("no healthy pool member")]
    NoHealthyPoolMember,
    #[error("internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AppError::InvalidApiKey => (
                StatusCode::UNAUTHORIZED,
                ErrorBody {
                    error: ErrorDetail {
                        message: "invalid api key".into(),
                        error_type: "authentication_error".into(),
                        code: "invalid_api_key".into(),
                    },
                },
            ),
            AppError::UpstreamUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorBody {
                    error: ErrorDetail {
                        message: "upstream unavailable".into(),
                        error_type: "service_unavailable_error".into(),
                        code: "upstream_unavailable".into(),
                    },
                },
            ),
            AppError::NoHealthyPoolMember => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorBody {
                    error: ErrorDetail {
                        message: "no healthy pool member".into(),
                        error_type: "service_unavailable_error".into(),
                        code: "no_healthy_pool_member".into(),
                    },
                },
            ),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorBody {
                    error: ErrorDetail {
                        message: "internal server error".into(),
                        error_type: "internal_server_error".into(),
                        code: "internal_error".into(),
                    },
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}
