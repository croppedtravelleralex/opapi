use crate::domain::release_record::ReleaseRecord;
use chrono::Utc;

pub fn draft(note: &str) -> ReleaseRecord {
    ReleaseRecord {
        id: format!("rel-{}", Utc::now().timestamp_millis()),
        version: "v0-draft".into(),
        note: note.into(),
    }
}
