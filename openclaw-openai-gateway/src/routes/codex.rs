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

fn query_i64(dsn: &str, sql: &str) -> Result<i64, String> {
    let conn = rusqlite::Connection::open(dsn).map_err(|e| e.to_string())?;
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|e| e.to_string())
}

fn query_optional_string(dsn: &str, sql: &str) -> Option<String> {
    let conn = rusqlite::Connection::open(dsn).ok()?;
    conn.query_row(sql, [], |row| row.get(0)).ok()
}
