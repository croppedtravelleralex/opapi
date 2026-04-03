use crate::domain::pool_member::PoolMember;
use rusqlite::{params, Connection};

#[derive(Clone)]
pub struct PoolMemberRepository {
    dsn: String,
}

impl PoolMemberRepository {
    pub fn new(dsn: String) -> Self {
        Self { dsn }
    }

    pub fn upsert(&self, member: &PoolMember) -> Result<(), String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO pool_members (
                id, child_account_id, pool_status, admission_level, weight,
                current_load, cooldown_until, last_success_at, last_failure_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(child_account_id) DO UPDATE SET
                pool_status = excluded.pool_status,
                admission_level = excluded.admission_level,
                weight = excluded.weight,
                current_load = excluded.current_load,
                cooldown_until = excluded.cooldown_until,
                last_success_at = excluded.last_success_at,
                last_failure_at = excluded.last_failure_at",
            params![
                member.id,
                member.child_account_id,
                member.pool_status,
                member.admission_level,
                member.weight,
                member.current_load,
                member.cooldown_until,
                member.last_success_at,
                member.last_failure_at,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_child_account_id(&self, child_account_id: &str) -> Result<Option<PoolMember>, String> {
        let conn = Connection::open(&self.dsn).map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT id, child_account_id, pool_status, admission_level, weight,
                        current_load, cooldown_until, last_success_at, last_failure_at
                 FROM pool_members
                 WHERE child_account_id = ?1",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query([child_account_id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            Ok(Some(PoolMember {
                id: row.get(0).map_err(|e| e.to_string())?,
                child_account_id: row.get(1).map_err(|e| e.to_string())?,
                pool_status: row.get(2).map_err(|e| e.to_string())?,
                admission_level: row.get(3).map_err(|e| e.to_string())?,
                weight: row.get(4).map_err(|e| e.to_string())?,
                current_load: row.get(5).map_err(|e| e.to_string())?,
                cooldown_until: row.get(6).map_err(|e| e.to_string())?,
                last_success_at: row.get(7).map_err(|e| e.to_string())?,
                last_failure_at: row.get(8).map_err(|e| e.to_string())?,
            }))
        } else {
            Ok(None)
        }
    }
}
