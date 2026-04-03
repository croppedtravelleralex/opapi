use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use openclaw_openai_gateway::{app::build_app, config::Config, state::AppState};
use serde_json::{json, Value};
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use tower::ServiceExt;

async fn test_app() -> (axum::Router, String) {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/openclaw-gateway-test-{}.sqlite3", unique);
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: db_path.clone(),
        codex_session_bridge_mode: "mock".into(),
        third_party_provider_id: None,
        third_party_base_url: None,
        third_party_api_key: None,
        third_party_model: None,
    };
    let state = Arc::new(AppState::new(config).await.unwrap());
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    let _ = conn.execute("ALTER TABLE quota_snapshots ADD COLUMN source_id TEXT", []);
    (build_app(state), db_path)
}

#[tokio::test]
async fn codex_quota_collect_persists_pool_member_after_admission() {
    let (app, db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-pool-1",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "5h 78% 7d 91% requests 12 tokens 3456 messages 8"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["persisted_pool_member"]["child_account_id"], "child-pool-1");
    assert_eq!(payload["persisted_pool_member"]["pool_status"], "active");
    assert_eq!(payload["persisted_pool_member"]["admission_level"], "green");
    assert_eq!(payload["persisted_pool_member"]["weight"], 100);

    let conn = rusqlite::Connection::open(db_path).unwrap();
    let row: (String, String, i64) = conn
        .query_row(
            "SELECT pool_status, admission_level, weight FROM pool_members WHERE child_account_id = 'child-pool-1'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(row.0, "active");
    assert_eq!(row.1, "green");
    assert_eq!(row.2, 100);
}

#[tokio::test]
async fn chat_uses_best_active_pool_member_headers() {
    let (app, db_path) = test_app().await;
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO pool_members (
            id, child_account_id, pool_status, admission_level, weight,
            current_load, cooldown_until, last_success_at, last_failure_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            "pool-green-1",
            "child-green-1",
            "active",
            "green",
            100_i64,
            0_i64,
            Option::<String>::None,
            Some("2026-04-03T09:00:00+08:00".to_string()),
            Option::<String>::None,
        ],
    ).unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_id, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        rusqlite::params![
            "snap-chat-1",
            "child-green-1",
            "2026-04-03T09:00:01+08:00",
            78.0_f64,
            91.0_f64,
            12_i64,
            3456_i64,
            8_i64,
            "codex-app",
            "/codex",
            0.96_f64,
            1_i64,
            Option::<String>::None,
        ],
    ).unwrap();

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
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("x-pool-child-account-id").unwrap(), "child-green-1");
    assert_eq!(response.headers().get("x-pool-admission-level").unwrap(), "green");
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["choices"][0]["message"]["content"], "codex routed via child-green-1 [green] source=codex-app page=/codex: mock-session-bridge source=codex-app page=/codex input=ping");
}

#[tokio::test]
async fn chat_returns_no_healthy_pool_member_when_pool_empty() {
    let (app, _db_path) = test_app().await;
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
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["error"]["code"], "no_healthy_pool_member");
}

#[tokio::test]
async fn responses_uses_best_active_pool_member_headers() {
    let (app, db_path) = test_app().await;
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO pool_members (
            id, child_account_id, pool_status, admission_level, weight,
            current_load, cooldown_until, last_success_at, last_failure_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            "pool-yellow-1",
            "child-yellow-1",
            "active",
            "yellow",
            30_i64,
            0_i64,
            Option::<String>::None,
            Some("2026-04-03T09:00:00+08:00".to_string()),
            Option::<String>::None,
        ],
    ).unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_id, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        rusqlite::params![
            "snap-response-1",
            "child-yellow-1",
            "2026-04-03T09:00:01+08:00",
            18.0_f64,
            84.0_f64,
            12_i64,
            3456_i64,
            8_i64,
            "codex-web",
            "/codex",
            0.88_f64,
            1_i64,
            Option::<String>::None,
        ],
    ).unwrap();

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
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("x-pool-child-account-id").unwrap(), "child-yellow-1");
    assert_eq!(response.headers().get("x-pool-admission-level").unwrap(), "yellow");
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["output"][0]["content"][0]["text"], "codex routed via child-yellow-1 [yellow] source=codex-web page=/codex: mock-session-bridge source=codex-web page=/codex input=ping");
}

#[tokio::test]
async fn codex_quota_collect_returns_green_admission_for_healthy_quota() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-green-1",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "5h 78% 7d 91% requests 12 tokens 3456 messages 8"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["admission"]["pool_status"], "active");
    assert_eq!(payload["admission"]["admission_level"], "green");
    assert_eq!(payload["admission"]["weight"], 100);
}

#[tokio::test]
async fn codex_quota_collect_returns_yellow_admission_for_low_quota() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-yellow-1",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "5h 18% 7d 84% requests 12 tokens 3456 messages 8"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["admission"]["pool_status"], "active");
    assert_eq!(payload["admission"]["admission_level"], "yellow");
    assert_eq!(payload["admission"]["weight"], 30);
}

#[tokio::test]
async fn codex_quota_collect_returns_red_admission_when_read_fails() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-red-1",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "hello nothing useful here"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["admission"]["pool_status"], "cooling");
    assert_eq!(payload["admission"]["admission_level"], "red");
    assert_eq!(payload["admission"]["weight"], 0);
}

#[tokio::test]
async fn codex_quota_collect_parses_and_persists_snapshot() {
    let (app, db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-app-1",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "5h 78% 7d 91% requests 12 tokens 3456 messages 8"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["data"]["child_account_id"], "child-app-1");
    assert_eq!(payload["data"]["quota_5h_percent"], 78.0);
    assert_eq!(payload["data"]["quota_7d_percent"], 91.0);
    assert_eq!(payload["data"]["request_count"], 12);
    assert_eq!(payload["data"]["token_count"], 3456);
    assert_eq!(payload["data"]["message_count"], 8);
    assert_eq!(payload["data"]["source_id"], "codex-app");
    assert_eq!(payload["data"]["read_ok"], true);

    let conn = rusqlite::Connection::open(db_path).unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM quota_snapshots WHERE child_account_id = 'child-app-1'", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn codex_quota_collect_marks_failure_when_no_signals_found() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/codex/quota/collect")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "child_account_id": "child-app-2",
                        "source_id": "codex-app",
                        "source_page": "/codex",
                        "page_text": "hello nothing useful here"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["data"]["read_ok"], false);
    assert_eq!(payload["data"]["error_reason"], "quota_signals_not_found");
}

#[tokio::test]
async fn codex_quota_sources_returns_app_and_web_sources() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/codex/quota-sources")
                .header("authorization", "Bearer sk-test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    let data = payload["data"].as_array().unwrap();
    assert!(data.iter().any(|item| item["id"] == "codex-app" && item["provider_id"] == "codex.app"));
    assert!(data.iter().any(|item| item["id"] == "codex-web" && item["provider_id"] == "codex.web"));
}

#[tokio::test]
async fn codex_quota_overview_returns_seeded_observation_stats() {
    let (app, db_path) = test_app().await;
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            "snap-1",
            "child-1",
            "2026-04-03T08:20:00+08:00",
            81.5_f64,
            92.0_f64,
            12_i64,
            3456_i64,
            7_i64,
            "/codex",
            0.96_f64,
            1_i64,
            Option::<String>::None,
        ],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            "snap-2",
            "child-2",
            "2026-04-03T08:21:00+08:00",
            Option::<f64>::None,
            Option::<f64>::None,
            Option::<i64>::None,
            Option::<i64>::None,
            Option::<i64>::None,
            "/codex",
            0.12_f64,
            0_i64,
            Some("dom_changed".to_string()),
        ],
    )
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/codex/quota-overview")
                .header("authorization", "Bearer sk-test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["data"]["sources_total"], 2);
    assert_eq!(payload["data"]["sources_enabled"], 2);
    assert_eq!(payload["data"]["observations_total"], 2);
    assert_eq!(payload["data"]["read_ok_total"], 1);
    assert_eq!(payload["data"]["read_failed_total"], 1);
    assert_eq!(payload["data"]["latest_observed_at"], "2026-04-03T08:21:00+08:00");
}

#[tokio::test]
async fn healthz_returns_ok() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn models_requires_auth() {
    let (app, _db_path) = test_app().await;
    let response = app
        .oneshot(Request::builder().uri("/v1/models").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn models_returns_list() {
    let (app, _db_path) = test_app().await;
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
    let (app, _db_path) = test_app().await;
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
async fn providers_can_include_imported_third_party_api_key_and_base_url() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/openclaw-gateway-import-test-{}.sqlite3", unique);
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: db_path,
        codex_session_bridge_mode: "mock".into(),
        third_party_provider_id: Some("api.openai-compatible-demo".into()),
        third_party_base_url: Some("https://example.com/v1".into()),
        third_party_api_key: Some("sk-demo-provider-key".into()),
        third_party_model: Some("gpt-4o-mini".into()),
    };
    let app = build_app(Arc::new(AppState::new(config).await.unwrap()));
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
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    let data = payload["data"].as_array().unwrap();
    assert!(data.iter().any(|item| {
        item["id"] == "api.openai-compatible-demo"
            && item["class"] == "Api"
            && item["base_url"] == "https://example.com/v1"
            && item["api_key_hint"] == "sk-dem***"
    }));
}

#[tokio::test]
async fn routing_explain_uses_capability_and_availability() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/openclaw-gateway-routing-test-{}.sqlite3", unique);
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: db_path,
        codex_session_bridge_mode: "mock".into(),
        third_party_provider_id: None,
        third_party_base_url: None,
        third_party_api_key: None,
        third_party_model: None,
    };
    let state = AppState::new(config).await.unwrap();
    let decision = openclaw_openai_gateway::routing::policy::decide_provider(
        "openclaw-default",
        &openclaw_openai_gateway::routing::policy::default_policy(),
        Some(&state),
    );
    let explain = openclaw_openai_gateway::observability::explain::explain(&decision);
    assert!(explain.contains("availability_status=available"));
    assert!(explain.contains("supports_responses_api=true"));
}

#[tokio::test]
async fn sqlite_file_is_seeded() {
    let (app, db_path) = test_app().await;
    let _ = app;
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
    let audit_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM audit_events", [], |row| row.get(0))
        .unwrap();
    let parent_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM parent_accounts", [], |row| row.get(0))
        .unwrap();
    let child_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM child_accounts", [], |row| row.get(0))
        .unwrap();
    let membership_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM space_memberships", [], |row| row.get(0))
        .unwrap();
    let invite_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM invite_tasks", [], |row| row.get(0))
        .unwrap();
    let quota_snapshot_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM quota_snapshots", [], |row| row.get(0))
        .unwrap();
    let pool_member_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM pool_members", [], |row| row.get(0))
        .unwrap();
    let proxy_key_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM proxy_api_keys", [], |row| row.get(0))
        .unwrap();
    assert!(model_count >= 1);
    assert!(provider_count >= 1);
    assert!(capability_count >= 1);
    assert!(availability_count >= 1);
    assert!(audit_count >= 0);
    assert!(parent_count >= 0);
    assert!(child_count >= 0);
    assert!(membership_count >= 0);
    assert!(invite_count >= 0);
    assert!(quota_snapshot_count >= 0);
    assert!(pool_member_count >= 0);
    assert!(proxy_key_count >= 0);
}

#[tokio::test]
async fn chat_openclaw_ws_bridge_mode_returns_upstream_unavailable_when_ws_unreachable() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/openclaw-gateway-ws-bridge-chat-test-{}.sqlite3", unique);
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: db_path.clone(),
        codex_session_bridge_mode: "openclaw-ws".into(),
        third_party_provider_id: None,
        third_party_base_url: None,
        third_party_api_key: None,
        third_party_model: None,
    };
    let state = Arc::new(AppState::new(config).await.unwrap());
    let app = build_app(state);
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    let _ = conn.execute("ALTER TABLE quota_snapshots ADD COLUMN source_id TEXT", []);
    conn.execute(
        "INSERT INTO pool_members (
            id, child_account_id, pool_status, admission_level, weight,
            current_load, cooldown_until, last_success_at, last_failure_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            "pool-ws-chat-1",
            "child-ws-chat-1",
            "active",
            "green",
            100_i64,
            0_i64,
            Option::<String>::None,
            Some("2026-04-03T10:00:00+08:00".to_string()),
            Option::<String>::None,
        ],
    ).unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_id, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        rusqlite::params![
            "snap-ws-chat-1",
            "child-ws-chat-1",
            "2026-04-03T10:00:01+08:00",
            80.0_f64,
            92.0_f64,
            12_i64,
            3456_i64,
            8_i64,
            "codex-app",
            "/codex",
            0.96_f64,
            1_i64,
            Option::<String>::None,
        ],
    ).unwrap();

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
async fn responses_openclaw_ws_bridge_mode_returns_upstream_unavailable_when_ws_unreachable() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = format!("/tmp/openclaw-gateway-ws-bridge-response-test-{}.sqlite3", unique);
    let config = Config {
        app_host: "127.0.0.1".into(),
        app_port: 18080,
        openclaw_ws_url: "ws://127.0.0.1:39999".into(),
        openclaw_api_timeout_ms: 50,
        api_keys: vec!["sk-test".into()],
        models: vec!["openclaw-default".into()],
        sqlite_path: db_path.clone(),
        codex_session_bridge_mode: "openclaw-ws".into(),
        third_party_provider_id: None,
        third_party_base_url: None,
        third_party_api_key: None,
        third_party_model: None,
    };
    let state = Arc::new(AppState::new(config).await.unwrap());
    let app = build_app(state);
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    let _ = conn.execute("ALTER TABLE quota_snapshots ADD COLUMN source_id TEXT", []);
    conn.execute(
        "INSERT INTO pool_members (
            id, child_account_id, pool_status, admission_level, weight,
            current_load, cooldown_until, last_success_at, last_failure_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            "pool-ws-response-1",
            "child-ws-response-1",
            "active",
            "green",
            100_i64,
            0_i64,
            Option::<String>::None,
            Some("2026-04-03T10:00:00+08:00".to_string()),
            Option::<String>::None,
        ],
    ).unwrap();
    conn.execute(
        "INSERT INTO quota_snapshots (
            id, child_account_id, observed_at, quota_5h_percent, quota_7d_percent,
            request_count, token_count, message_count, source_id, source_page, confidence, read_ok, error_reason
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        rusqlite::params![
            "snap-ws-response-1",
            "child-ws-response-1",
            "2026-04-03T10:00:01+08:00",
            80.0_f64,
            92.0_f64,
            12_i64,
            3456_i64,
            8_i64,
            "codex-web",
            "/codex",
            0.96_f64,
            1_i64,
            Option::<String>::None,
        ],
    ).unwrap();

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
