use crate::domain::pool_member::PoolMember;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct RoutedPoolMember {
    pub child_account_id: String,
    pub pool_status: String,
    pub admission_level: String,
    pub weight: i64,
}

#[derive(Clone)]
pub struct PoolRouter {
    dsn: String,
}

impl PoolRouter {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn pick_best_active_member(&self) -> Result<Option<RoutedPoolMember>, String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT child_account_id, pool_status, admission_level, weight
                 FROM pool_members
                 WHERE pool_status = 'active' AND weight > 0
                 ORDER BY weight DESC, child_account_id ASC
                 LIMIT 1",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            Ok(Some(RoutedPoolMember {
                child_account_id: row.get(0).map_err(|e| e.to_string())?,
                pool_status: row.get(1).map_err(|e| e.to_string())?,
                admission_level: row.get(2).map_err(|e| e.to_string())?,
                weight: row.get(3).map_err(|e| e.to_string())?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl From<PoolMember> for RoutedPoolMember {
    fn from(value: PoolMember) -> Self {
        Self {
            child_account_id: value.child_account_id,
            pool_status: value.pool_status,
            admission_level: value.admission_level,
            weight: value.weight,
        }
    }
}
