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
    pub status: String,
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

    Json(AutoRegisterCodexAccountResponse {
        parent_account_id,
        child_account_id,
        invite_task_id,
        space_membership_id,
        proxy_key_id,
        proxy_key_plaintext,
        status: "pending_invite".into(),
    })
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
