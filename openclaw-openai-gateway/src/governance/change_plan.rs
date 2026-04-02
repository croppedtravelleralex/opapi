use crate::domain::change_plan::ChangePlan;
use chrono::Utc;

pub fn draft(title: &str) -> ChangePlan {
    ChangePlan {
        id: format!("chg-{}", Utc::now().timestamp_millis()),
        title: title.into(),
        status: "draft".into(),
    }
}
