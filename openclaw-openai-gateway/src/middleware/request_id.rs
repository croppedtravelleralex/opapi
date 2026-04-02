use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn request_id_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    if let Ok(value) = HeaderValue::from_str(&request_id) {
        req.headers_mut().insert("x-request-id", value);
    }
    next.run(req).await
}
