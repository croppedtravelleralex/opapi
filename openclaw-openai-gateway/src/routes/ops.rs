use crate::{routes::codex, state::AppState};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
};
use rusqlite::params;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct OpsOverviewResponse {
    pub service_mode: &'static str,
    pub remote_access_mode: &'static str,
    pub public_exposure: bool,
    pub ssh_tunnel_recommended: bool,
    pub listeners: ListenerSummary,
    pub registration_tasks: StatusBreakdown,
    pub verification_tasks: StatusBreakdown,
    pub mailbox_pool: MailboxPoolStats,
    pub automation_targets: StatusBreakdown,
    pub pool_members: StatusBreakdown,
    pub quota_snapshots: QuotaSnapshotStats,
    pub recommendations: Vec<String>,
}

#[derive(Serialize)]
pub struct ListenerSummary {
    pub dashboard_bind: &'static str,
    pub access_pattern: &'static str,
}

#[derive(Serialize)]
pub struct StatusBreakdown {
    pub total: i64,
    pub active_like: i64,
    pub blocked_like: i64,
    pub failed_like: i64,
    pub details: Vec<StatusCount>,
}

#[derive(Serialize)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct MailboxPoolStats {
    pub total: i64,
    pub active: i64,
    pub frozen: i64,
    pub cooling: i64,
    pub average_quality_score: f64,
    pub seed_count: i64,
    pub scaled_count: i64,
    pub premium_count: i64,
}

#[derive(Serialize)]
pub struct QuotaSnapshotStats {
    pub total: i64,
    pub read_ok_total: i64,
    pub read_failed_total: i64,
}

pub async fn get_ops_overview(State(state): State<Arc<AppState>>) -> Json<OpsOverviewResponse> {
    let conn = rusqlite::Connection::open(&state.config.sqlite_path).unwrap();

    let registration_tasks = status_breakdown(&conn, "registration_tasks");
    let verification_tasks = status_breakdown(&conn, "verification_tasks");
    let automation_targets = status_breakdown(&conn, "automation_targets");
    let pool_members = status_breakdown(&conn, "pool_members");

    let mailbox_pool = MailboxPoolStats {
        total: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes"),
        active: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE status = 'active'"),
        frozen: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE status = 'frozen'"),
        cooling: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE status = 'cooling'"),
        average_quality_score: scalar_f64(&conn, "SELECT COALESCE(AVG(quality_score), 0) FROM managed_mailboxes"),
        seed_count: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE expansion_tier = 'seed'"),
        scaled_count: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE expansion_tier = 'scaled'"),
        premium_count: scalar_i64(&conn, "SELECT COUNT(*) FROM managed_mailboxes WHERE expansion_tier = 'premium'"),
    };

    let quota_snapshots = QuotaSnapshotStats {
        total: scalar_i64(&conn, "SELECT COUNT(*) FROM quota_snapshots"),
        read_ok_total: scalar_i64(&conn, "SELECT COUNT(*) FROM quota_snapshots WHERE read_ok = 1"),
        read_failed_total: scalar_i64(&conn, "SELECT COUNT(*) FROM quota_snapshots WHERE read_ok = 0"),
    };

    Json(OpsOverviewResponse {
        service_mode: "headless-api-worker",
        remote_access_mode: "ssh-tunnel-only",
        public_exposure: false,
        ssh_tunnel_recommended: true,
        listeners: ListenerSummary {
            dashboard_bind: "127.0.0.1",
            access_pattern: "ssh -L 8088:127.0.0.1:<service-port> <host>",
        },
        registration_tasks,
        verification_tasks,
        mailbox_pool,
        automation_targets,
        pool_members,
        quota_snapshots,
        recommendations: vec![
            "dashboard 只绑定 127.0.0.1，通过 SSH 隧道访问，不直接公网暴露".into(),
            "继续补 scheduler，把 worker / mailbox poll / tiering 串成常驻循环".into(),
            "冻结 GUI / 重控制台方向，保持轻量运维面板".into(),
        ],
    })
}

pub async fn get_ops_dashboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let overview = get_ops_overview(State(state)).await.0;
    let html = format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset=\"utf-8\" />
  <title>opapi dashboard</title>
  <style>
    body {{ font-family: sans-serif; background:#0b1020; color:#e5e7eb; margin:24px; }}
    .grid {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(260px,1fr)); gap:16px; }}
    .card {{ background:#121933; border:1px solid #24304f; border-radius:12px; padding:16px; }}
    h1,h2,h3 {{ margin:0 0 12px 0; }}
    .muted {{ color:#94a3b8; font-size:14px; }}
    .num {{ font-size:28px; font-weight:700; }}
    ul {{ padding-left:18px; }}
    code {{ background:#111827; padding:2px 6px; border-radius:6px; }}
  </style>
</head>
<body>
  <h1>opapi 轻量运维面板</h1>
  <div class=\"muted\">仅建议通过 SSH 隧道访问，不直接公网开放。</div>
  <div class=\"grid\" style=\"margin-top:16px\">
    <div class=\"card\"><h3>服务形态</h3><div class=\"num\">{}</div><div class=\"muted\">remote: {}</div></div>
    <div class=\"card\"><h3>注册任务</h3><div class=\"num\">{}</div><div class=\"muted\">active-like {} / blocked-like {} / failed-like {}</div></div>
    <div class=\"card\"><h3>验证任务</h3><div class=\"num\">{}</div><div class=\"muted\">active-like {} / blocked-like {} / failed-like {}</div></div>
    <div class=\"card\"><h3>邮箱池</h3><div class=\"num\">{}</div><div class=\"muted\">active {} / frozen {} / avg quality {:.1}</div></div>
    <div class=\"card\"><h3>额度快照</h3><div class=\"num\">{}</div><div class=\"muted\">ok {} / failed {}</div></div>
    <div class=\"card\"><h3>池成员</h3><div class=\"num\">{}</div><div class=\"muted\">active-like {} / blocked-like {} / failed-like {}</div></div>
  </div>
  <div class=\"card\" style=\"margin-top:16px\">
    <h3>SSH 访问方式</h3>
    <div class=\"muted\">{}</div>
    <p><code>{}</code></p>
  </div>
  <div class=\"card\" style=\"margin-top:16px\">
    <h3>建议</h3>
    <ul>{}</ul>
  </div>
</body>
</html>"#,
        overview.service_mode,
        overview.remote_access_mode,
        overview.registration_tasks.total,
        overview.registration_tasks.active_like,
        overview.registration_tasks.blocked_like,
        overview.registration_tasks.failed_like,
        overview.verification_tasks.total,
        overview.verification_tasks.active_like,
        overview.verification_tasks.blocked_like,
        overview.verification_tasks.failed_like,
        overview.mailbox_pool.total,
        overview.mailbox_pool.active,
        overview.mailbox_pool.frozen,
        overview.mailbox_pool.average_quality_score,
        overview.quota_snapshots.total,
        overview.quota_snapshots.read_ok_total,
        overview.quota_snapshots.read_failed_total,
        overview.pool_members.total,
        overview.pool_members.active_like,
        overview.pool_members.blocked_like,
        overview.pool_members.failed_like,
        "dashboard 绑定在 127.0.0.1，远程使用 SSH 端口转发",
        overview.listeners.access_pattern,
        overview
            .recommendations
            .iter()
            .map(|x| format!("<li>{}</li>", x))
            .collect::<Vec<_>>()
            .join(""),
    );
    Html(html)
}

#[derive(Serialize)]
pub struct SchedulerTickResponse {
    pub registration_worker: codex::RunRegistrationWorkerResponse,
    pub mailbox_poll: codex::MailboxPollRunResponse,
    pub mailbox_tiering: codex::MailboxTieringRunResponse,
    pub dead_letter_recover: codex::RecoverDeadLettersResponse,
}

pub async fn run_scheduler_tick(State(state): State<Arc<AppState>>) -> Json<SchedulerTickResponse> {
    let registration_worker = codex::run_registration_worker(
        State(state.clone()),
        Json(codex::RegistrationWorkerRunRequest { max_tasks: Some(8) }),
    )
    .await
    .0;
    let mailbox_poll = codex::poll_managed_mailboxes(State(state.clone())).await.0;
    let mailbox_tiering = codex::run_mailbox_tiering(State(state.clone())).await.0;
    let dead_letter_recover = codex::recover_dead_letters(State(state)).await.0;
    Json(SchedulerTickResponse {
        registration_worker,
        mailbox_poll,
        mailbox_tiering,
        dead_letter_recover,
    })
}

fn scalar_i64(conn: &rusqlite::Connection, sql: &str) -> i64 {
    conn.query_row(sql, [], |row| row.get(0)).unwrap_or(0)
}

fn scalar_f64(conn: &rusqlite::Connection, sql: &str) -> f64 {
    conn.query_row(sql, [], |row| row.get(0)).unwrap_or(0.0)
}

fn status_breakdown(conn: &rusqlite::Connection, table: &str) -> StatusBreakdown {
    let total = scalar_i64(conn, &format!("SELECT COUNT(*) FROM {table}"));
    let details = collect_status_counts(conn, table);
    let active_like = details
        .iter()
        .filter(|item| matches!(item.status.as_str(), "pending" | "running" | "active" | "verified" | "automatable" | "completed"))
        .map(|item| item.count)
        .sum();
    let blocked_like = details
        .iter()
        .filter(|item| matches!(item.status.as_str(), "blocked" | "retry_wait" | "waiting_mailbox" | "cooling" | "frozen" | "needs_optimization"))
        .map(|item| item.count)
        .sum();
    let failed_like = details
        .iter()
        .filter(|item| matches!(item.status.as_str(), "dead_letter" | "failed" | "read_failed"))
        .map(|item| item.count)
        .sum();

    StatusBreakdown { total, active_like, blocked_like, failed_like, details }
}

fn collect_status_counts(conn: &rusqlite::Connection, table: &str) -> Vec<StatusCount> {
    let mut stmt = match conn.prepare(&format!("SELECT status, COUNT(*) FROM {table} GROUP BY status ORDER BY COUNT(*) DESC, status ASC")) {
        Ok(stmt) => stmt,
        Err(_) => return vec![],
    };
    let rows = stmt
        .query_map(params![], |row| {
            Ok(StatusCount { status: row.get(0)?, count: row.get(1)? })
        })
        .unwrap();
    rows.filter_map(Result::ok).collect()
}
