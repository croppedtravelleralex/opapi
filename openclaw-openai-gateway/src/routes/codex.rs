use crate::{
    domain::codex_quota_source::{default_codex_quota_sources, CodexQuotaSource},
    state::AppState,
};
use axum::{extract::State, Json};
use serde::Serialize;
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
