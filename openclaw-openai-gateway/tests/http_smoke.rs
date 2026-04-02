use axum::body::Body;
use axum::http::{Request, StatusCode};
use openclaw_openai_gateway::{app::build_app, config::Config, state::AppState};
use serde_json::json;
use std::{fs, sync::Arc};
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let _ = fs::remove_file("/tmp/openclaw-gateway-test.sqlite3");
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: "/tmp/openclaw-gateway-test.sqlite3".into(),
    };
    let state = Arc::new(AppState::new(config).await.unwrap());
    build_app(state)
}

#[tokio::test]
async fn healthz_returns_ok() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn models_requires_auth() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::builder().uri("/v1/models").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn models_returns_list() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/models")
                .header("authorization", "Bearer sk-test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn providers_returns_list() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/providers")
                .header("authorization", "Bearer sk-test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn sqlite_file_is_seeded() {
    let _app = test_app().await;
    let db_path = "/tmp/openclaw-gateway-test.sqlite3";
    let conn = rusqlite::Connection::open(db_path).unwrap();
    let model_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM model_catalog", [], |row| row.get(0))
        .unwrap();
    let provider_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM providers", [], |row| row.get(0))
        .unwrap();
    let capability_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM provider_capabilities", [], |row| row.get(0))
        .unwrap();
    let availability_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM model_availability", [], |row| row.get(0))
        .unwrap();
    assert!(model_count >= 1);
    assert!(provider_count >= 1);
    assert!(capability_count >= 1);
    assert!(availability_count >= 1);
}

#[tokio::test]
async fn chat_returns_upstream_unavailable_when_gateway_unreachable() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "model": "openclaw-default",
                        "messages": [{"role": "user", "content": "ping"}]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn responses_returns_upstream_unavailable_when_gateway_unreachable() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/responses")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "model": "openclaw-default",
                        "input": "ping"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
