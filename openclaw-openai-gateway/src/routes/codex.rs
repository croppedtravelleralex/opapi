use crate::{
    codex::{
        admission::decide_from_snapshot,
        collector::CodexQuotaCollector,
        pool_repo::PoolMemberRepository,
    },
    domain::codex_quota_source::{default_codex_quota_sources, CodexQuotaSource},
    state::AppState,
};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct CodexQuotaSourceList {
    pub data: Vec<CodexQuotaSourceItem>,
}

#[derive(Serialize)]
pub struct CodexQuotaSourceItem {
    pub id: String,
    pub kind: String,
    pub enabled: bool,
    pub display_name: String,
    pub provider_id: String,
    pub base_url: Option<String>,
    pub observation_path: String,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct CodexQuotaOverview {
    pub sources_total: usize,
    pub sources_enabled: usize,
    pub observations_total: i64,
    pub read_ok_total: i64,
    pub read_failed_total: i64,
    pub latest_observed_at: Option<String>,
}

#[derive(Serialize)]
pub struct CodexQuotaOverviewResponse {
    pub data: CodexQuotaOverview,
}

#[derive(Deserialize)]
pub struct AutoRegisterCodexAccountRequest {
    pub parent_email: String,
    pub child_email: String,
    pub space_name: Option<String>,
    pub fingerprint_profile_id: Option<String>,
    pub proxy_key_label: Option<String>,
    pub allowed_models: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct AutoRegisterCodexAccountResponse {
    pub parent_account_id: String,
    pub child_account_id: String,
    pub invite_task_id: String,
    pub space_membership_id: String,
    pub proxy_key_id: String,
    pub proxy_key_plaintext: String,
    pub registration_tasks: Vec<RegistrationTaskItem>,
    pub status: String,
}

#[derive(Serialize)]
pub struct RegistrationTaskItem {
    pub id: String,
    pub task_type: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct DispatchRegistrationTaskResponse {
    pub id: String,
    pub task_type: String,
    pub status: String,
    pub result: serde_json::Value,
}

#[derive(Serialize)]
pub struct RunRegistrationWorkerResponse {
    pub dispatched: usize,
    pub results: Vec<DispatchRegistrationTaskResponse>,
}

#[derive(Deserialize)]
pub struct CollectCodexQuotaRequest {
    pub child_account_id: String,
    pub source_id: String,
    pub source_page: String,
    pub page_text: String,
    pub page_html: Option<String>,
    pub snapshot_ref: Option<String>,
    pub session_namespace: Option<String>,
    pub session_key_hint: Option<String>,
}

#[derive(Serialize)]
pub struct CollectCodexQuotaResponse {
    pub data: crate::domain::quota_snapshot::QuotaSnapshot,
    pub admission: CodexAdmissionItem,
    pub persisted_pool_member: CodexPersistedPoolMemberItem,
}

#[derive(Serialize)]
pub struct CodexAdmissionItem {
    pub pool_status: String,
    pub admission_level: String,
    pub weight: i64,
    pub reasons: Vec<String>,
}

#[derive(Serialize)]
pub struct CodexPersistedPoolMemberItem {
    pub child_account_id: String,
    pub pool_status: String,
    pub admission_level: String,
    pub weight: i64,
}

pub async fn auto_register_codex_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AutoRegisterCodexAccountRequest>,
) -> Json<AutoRegisterCodexAccountResponse> {
    let conn = rusqlite::Connection::open(&state.config.sqlite_path).expect("open sqlite");
    let now = chrono::Utc::now().to_rfc3339();
    let parent_account_id = format!("parent:{}", sanitize_key_fragment(&payload.parent_email));
    let child_account_id = format!("child:{}", sanitize_key_fragment(&payload.child_email));
    let invite_task_id = format!("invite:{}:{}", parent_account_id, child_account_id);
    let space_membership_id = format!("membership:{}:{}", parent_account_id, child_account_id);
    let proxy_key_id = format!("proxy-key:{}", child_account_id);
    let proxy_key_plaintext = format!(
        "opapi_{}_{}",
        sanitize_key_fragment(&payload.child_email),
        chrono::Utc::now().timestamp_millis()
    );
    let allowed_models = serde_json::to_string(
        &payload
            .allowed_models
            .clone()
            .unwrap_or_else(|| state.config.models.clone()),
    )
    .unwrap_or_else(|_| "[]".into());

    let _ = conn.execute(
        "INSERT OR REPLACE INTO parent_accounts (
            id, email, space_name, status, fingerprint_profile_id, invite_enabled, risk_level, last_login_at
        ) VALUES (?1, ?2, ?3, 'active', ?4, 1, 'normal', ?5)",
        rusqlite::params![
            parent_account_id,
            payload.parent_email,
            payload.space_name.unwrap_or_else(|| "Codex Auto Space".into()),
            payload.fingerprint_profile_id,
            now,
        ],
    );

    let _ = conn.execute(
        "INSERT OR REPLACE INTO child_accounts (
            id, email, parent_account_id, status, space_verified, pool_status, risk_level, fingerprint_profile_id, last_login_at
        ) VALUES (?1, ?2, ?3, 'warming', 0, 'new', 'normal', ?4, ?5)",
        rusqlite::params![
            child_account_id,
            payload.child_email,
            parent_account_id,
            payload.fingerprint_profile_id,
            now,
        ],
    );

    let _ = conn.execute(
        "INSERT OR REPLACE INTO invite_tasks (
            id, parent_account_id, child_account_id, status, sent_at, accepted_at, error_reason
        ) VALUES (?1, ?2, ?3, 'pending', ?4, NULL, NULL)",
        rusqlite::params![invite_task_id, parent_account_id, child_account_id, now],
    );

    let _ = conn.execute(
        "INSERT OR REPLACE INTO space_memberships (
            id, parent_account_id, child_account_id, joined, verified, verified_at
        ) VALUES (?1, ?2, ?3, 0, 0, NULL)",
        rusqlite::params![space_membership_id, parent_account_id, child_account_id],
    );

    let _ = conn.execute(
        "INSERT OR REPLACE INTO proxy_api_keys (
            id, label, hashed_key, owner, status, rate_limit, quota_limit, allowed_models_json
        ) VALUES (?1, ?2, ?3, ?4, 'active', 60, 1000, ?5)",
        rusqlite::params![
            proxy_key_id,
            payload
                .proxy_key_label
                .unwrap_or_else(|| format!("auto-register:{}", child_account_id)),
            proxy_key_plaintext,
            child_account_id,
            allowed_models,
        ],
    );

    let registration_tasks = vec![
        ("register-account", serde_json::json!({"fingerprint_profile_id": payload.fingerprint_profile_id, "child_email": payload.child_email})),
        ("accept-invite", serde_json::json!({"invite_task_id": invite_task_id, "parent_email": payload.parent_email})),
        ("collect-quota", serde_json::json!({"source_id": "codex-app", "source_page": "/codex"})),
        ("warmup-pool", serde_json::json!({"target_pool_status": "active"})),
    ]
    .into_iter()
    .map(|(task_type, payload_json)| {
        let task_id = format!("registration-task:{}:{}", child_account_id, task_type);
        let _ = conn.execute(
            "INSERT OR REPLACE INTO registration_tasks (
                id, parent_account_id, child_account_id, task_type, status, payload_json, result_json, queued_at, started_at, finished_at, error_reason
            ) VALUES (?1, ?2, ?3, ?4, 'pending', ?5, NULL, ?6, NULL, NULL, NULL)",
            rusqlite::params![
                task_id,
                parent_account_id,
                child_account_id,
                task_type,
                payload_json.to_string(),
                now,
            ],
        );
        RegistrationTaskItem {
            id: task_id,
            task_type: task_type.into(),
            status: "pending".into(),
        }
    })
    .collect::<Vec<_>>();

    Json(AutoRegisterCodexAccountResponse {
        parent_account_id,
        child_account_id,
        invite_task_id,
        space_membership_id,
        proxy_key_id,
        proxy_key_plaintext,
        registration_tasks,
        status: "pending_invite".into(),
    })
}

pub async fn run_registration_worker(
    State(state): State<Arc<AppState>>,
) -> Json<RunRegistrationWorkerResponse> {
    let mut results = Vec::new();
    for _ in 0..8 {
        let item = dispatch_registration_task_once(&state);
        if item.status == "idle" {
            break;
        }
        results.push(item);
    }
    Json(RunRegistrationWorkerResponse {
        dispatched: results.len(),
        results,
    })
}

pub async fn dispatch_registration_task(
    State(state): State<Arc<AppState>>,
) -> Json<DispatchRegistrationTaskResponse> {
    Json(dispatch_registration_task_once(&state))
}

fn dispatch_registration_task_once(
    state: &Arc<AppState>,
) -> DispatchRegistrationTaskResponse {
    let conn = rusqlite::Connection::open(&state.config.sqlite_path).expect("open sqlite");
    let now = chrono::Utc::now().to_rfc3339();
    let next = conn.query_row(
        "SELECT id, parent_account_id, child_account_id, task_type, payload_json
         FROM registration_tasks
         WHERE status = 'pending'
         ORDER BY queued_at ASC, id ASC
         LIMIT 1",
        [],
        |row| Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        )),
    );

    let (id, parent_account_id, child_account_id, task_type, payload_json) =
        next.unwrap_or_else(|_| (
            "registration-task:none".into(),
            "none".into(),
            "none".into(),
            "idle".into(),
            "{}".into(),
        ));

    let result = match task_type.as_str() {
        "register-account" => {
            let _ = conn.execute(
                "UPDATE child_accounts SET status = 'registered', last_login_at = ?2 WHERE id = ?1",
                rusqlite::params![child_account_id, now],
            );
            serde_json::json!({
                "runner": "fingerprint-browser",
                "provider": state.config.fingerprint_browser_provider.clone().unwrap_or_else(|| "pending-api".into()),
                "base_url": state.config.fingerprint_browser_base_url,
                "api_key_configured": state.config.fingerprint_browser_api_key.as_ref().map(|v| !v.is_empty()).unwrap_or(false),
                "action": "register-account",
                "payload": serde_json::from_str::<serde_json::Value>(&payload_json).unwrap_or_default()
            })
        }
        "accept-invite" => {
            let _ = conn.execute(
                "UPDATE invite_tasks SET status = 'accepted', accepted_at = ?2 WHERE child_account_id = ?1",
                rusqlite::params![child_account_id, now],
            );
            let _ = conn.execute(
                "UPDATE space_memberships SET joined = 1, verified = 1, verified_at = ?2 WHERE child_account_id = ?1",
                rusqlite::params![child_account_id, now],
            );
            serde_json::json!({
                "runner": "fingerprint-browser",
                "provider": state.config.fingerprint_browser_provider.clone().unwrap_or_else(|| "pending-api".into()),
                "base_url": state.config.fingerprint_browser_base_url,
                "api_key_configured": state.config.fingerprint_browser_api_key.as_ref().map(|v| !v.is_empty()).unwrap_or(false),
                "action": "accept-invite",
                "parent_account_id": parent_account_id
            })
        }
        "collect-quota" => {
            let collector = CodexQuotaCollector::new(state.config.sqlite_path.clone());
            let snapshot = collector.collect_from_page_text(
                crate::codex::parser::CodexQuotaPageInput {
                    child_account_id: child_account_id.clone(),
                    source_id: "codex-app".into(),
                    source_page: "/codex".into(),
                    page_text: "5h 82% 7d 94% requests 8 tokens 2048 messages 6".into(),
                    page_html: None,
                    snapshot_ref: Some("auto-register-warmup".into()),
                },
                crate::codex::collector::CodexAppSessionInput {
                    session_namespace: Some(format!("auto-register:{}", child_account_id)),
                    session_key_hint: Some(format!("bootstrap:{}", child_account_id)),
                },
            ).unwrap();
            let decision = decide_from_snapshot(&snapshot);
            let pool_repo = PoolMemberRepository::new(state.config.sqlite_path.clone());
            let _ = pool_repo.upsert(&decision.pool_member);
            serde_json::json!({"runner": "quota-collector", "snapshot_id": snapshot.id, "pool_status": decision.pool_member.pool_status, "admission_level": decision.pool_member.admission_level})
        }
        "warmup-pool" => {
            let _ = conn.execute(
                "UPDATE child_accounts SET status = 'warm', pool_status = 'active' WHERE id = ?1",
                rusqlite::params![child_account_id],
            );
            serde_json::json!({"runner": "pool-warmer", "target_pool_status": "active"})
        }
        _ => serde_json::json!({"runner": "noop", "status": "idle"}),
    };

    if task_type != "idle" {
        let _ = conn.execute(
            "UPDATE registration_tasks
             SET status = 'completed', started_at = COALESCE(started_at, ?2), finished_at = ?2, result_json = ?3
             WHERE id = ?1",
            rusqlite::params![id, now, result.to_string()],
        );
    }

    DispatchRegistrationTaskResponse {
        id,
        task_type,
        status: if result["status"] == "idle" { "idle".into() } else { "completed".into() },
        result,
    }
}

pub async fn collect_codex_quota(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CollectCodexQuotaRequest>,
) -> Json<CollectCodexQuotaResponse> {
    let collector = CodexQuotaCollector::new(state.config.sqlite_path.clone());
    let snapshot = collector
        .collect_from_page_text(
            crate::codex::parser::CodexQuotaPageInput {
                child_account_id: payload.child_account_id,
                source_id: payload.source_id,
                source_page: payload.source_page,
                page_text: payload.page_text,
                page_html: payload.page_html,
                snapshot_ref: payload.snapshot_ref,
            },
            crate::codex::collector::CodexAppSessionInput {
                session_namespace: payload.session_namespace,
                session_key_hint: payload.session_key_hint,
            },
        )
        .unwrap_or_else(|error_reason| crate::domain::quota_snapshot::QuotaSnapshot {
            id: "quota-collect-failed".into(),
            child_account_id: "unknown".into(),
            observed_at: chrono::Utc::now().to_rfc3339(),
            quota_5h_percent: None,
            quota_7d_percent: None,
            request_count: None,
            token_count: None,
            message_count: None,
            source_id: Some("codex-app".into()),
            source_page: Some("/codex".into()),
            confidence: Some(0.0),
            read_ok: false,
            error_reason: Some(error_reason),
        });
    let decision = decide_from_snapshot(&snapshot);
    let pool_repo = PoolMemberRepository::new(state.config.sqlite_path.clone());
    let _ = pool_repo.upsert(&decision.pool_member);
    let persisted = pool_repo
        .get_by_child_account_id(&decision.pool_member.child_account_id)
        .ok()
        .flatten()
        .unwrap_or_else(|| decision.pool_member.clone());

    Json(CollectCodexQuotaResponse {
        data: snapshot,
        admission: CodexAdmissionItem {
            pool_status: decision.pool_member.pool_status.clone(),
            admission_level: decision.pool_member.admission_level.clone(),
            weight: decision.pool_member.weight,
            reasons: decision.reasons,
        },
        persisted_pool_member: CodexPersistedPoolMemberItem {
            child_account_id: persisted.child_account_id,
            pool_status: persisted.pool_status,
            admission_level: persisted.admission_level,
            weight: persisted.weight,
        },
    })
}

pub async fn list_codex_quota_sources(
    State(_state): State<Arc<AppState>>,
) -> Json<CodexQuotaSourceList> {
    let data = default_codex_quota_sources()
        .into_iter()
        .map(map_source)
        .collect();

    Json(CodexQuotaSourceList { data })
}

pub async fn get_codex_quota_overview(
    State(state): State<Arc<AppState>>,
) -> Json<CodexQuotaOverviewResponse> {
    let observations_total = query_i64(
        &state.config.sqlite_path,
        "SELECT COUNT(*) FROM quota_snapshots",
    )
    .unwrap_or(0);
    let read_ok_total = query_i64(
        &state.config.sqlite_path,
        "SELECT COUNT(*) FROM quota_snapshots WHERE read_ok = 1",
    )
    .unwrap_or(0);
    let read_failed_total = query_i64(
        &state.config.sqlite_path,
        "SELECT COUNT(*) FROM quota_snapshots WHERE read_ok = 0",
    )
    .unwrap_or(0);
    let latest_observed_at = query_optional_string(
        &state.config.sqlite_path,
        "SELECT observed_at FROM quota_snapshots ORDER BY observed_at DESC LIMIT 1",
    );

    let sources = default_codex_quota_sources();
    let sources_total = sources.len();
    let sources_enabled = sources.iter().filter(|item| item.enabled).count();

    Json(CodexQuotaOverviewResponse {
        data: CodexQuotaOverview {
            sources_total,
            sources_enabled,
            observations_total,
            read_ok_total,
            read_failed_total,
            latest_observed_at,
        },
    })
}

fn map_source(source: CodexQuotaSource) -> CodexQuotaSourceItem {
    CodexQuotaSourceItem {
        id: source.id,
        kind: format!("{:?}", source.kind),
        enabled: source.enabled,
        display_name: source.display_name,
        provider_id: source.provider_id,
        base_url: source.base_url,
        observation_path: source.observation_path,
        notes: source.notes,
    }
}

fn sanitize_key_fragment(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn query_i64(dsn: &str, sql: &str) -> Result<i64, String> {
    let conn = rusqlite::Connection::open(dsn).map_err(|e| e.to_string())?;
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|e| e.to_string())
}

fn query_optional_string(dsn: &str, sql: &str) -> Option<String> {
    let conn = rusqlite::Connection::open(dsn).ok()?;
    conn.query_row(sql, [], |row| row.get(0)).ok()
}
