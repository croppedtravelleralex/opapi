use crate::domain::config_snapshot::ConfigSnapshot;
use chrono::Utc;

pub fn capture(summary: &str, source: &str) -> ConfigSnapshot {
    ConfigSnapshot {
        id: format!("cfg-{}", Utc::now().timestamp_millis()),
        summary: summary.into(),
        source: source.into(),
    }
}
