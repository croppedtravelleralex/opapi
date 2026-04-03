use crate::domain::{pool_member::PoolMember, quota_snapshot::QuotaSnapshot};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AdmissionDecision {
    pub pool_member: PoolMember,
    pub reasons: Vec<String>,
}

pub fn decide_from_snapshot(snapshot: &QuotaSnapshot) -> AdmissionDecision {
    let mut reasons = Vec::new();

    if !snapshot.read_ok {
        reasons.push("quota_read_failed".into());
        return AdmissionDecision {
            pool_member: PoolMember {
                id: format!("pool-{}", Uuid::new_v4()),
                child_account_id: snapshot.child_account_id.clone(),
                pool_status: "cooling".into(),
                admission_level: "red".into(),
                weight: 0,
                current_load: 0,
                cooldown_until: Some((Utc::now() + Duration::minutes(30)).to_rfc3339()),
                last_success_at: None,
                last_failure_at: Some(Utc::now().to_rfc3339()),
            },
            reasons,
        };
    }

    let quota_5h = snapshot.quota_5h_percent.unwrap_or(0.0);
    let quota_7d = snapshot.quota_7d_percent.unwrap_or(0.0);
    let requests = snapshot.request_count.unwrap_or(0);

    let (pool_status, admission_level, weight) = if quota_5h < 5.0 || quota_7d < 5.0 {
        reasons.push("quota_below_5_percent".into());
        ("cooling", "red", 0)
    } else if quota_5h < 20.0 || quota_7d < 20.0 || requests > 200 {
        if quota_5h < 20.0 || quota_7d < 20.0 {
            reasons.push("quota_low".into());
        }
        if requests > 200 {
            reasons.push("request_rate_high".into());
        }
        ("active", "yellow", 30)
    } else {
        reasons.push("quota_healthy".into());
        ("active", "green", 100)
    };

    AdmissionDecision {
        pool_member: PoolMember {
            id: format!("pool-{}", Uuid::new_v4()),
            child_account_id: snapshot.child_account_id.clone(),
            pool_status: pool_status.into(),
            admission_level: admission_level.into(),
            weight,
            current_load: 0,
            cooldown_until: if pool_status == "cooling" {
                Some((Utc::now() + Duration::minutes(30)).to_rfc3339())
            } else {
                None
            },
            last_success_at: if pool_status == "active" {
                Some(Utc::now().to_rfc3339())
            } else {
                None
            },
            last_failure_at: if pool_status == "cooling" {
                Some(Utc::now().to_rfc3339())
            } else {
                None
            },
        },
        reasons,
    }
}
