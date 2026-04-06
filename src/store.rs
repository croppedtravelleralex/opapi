use anyhow::{Context, Result};
use serde::Serialize;
use sqlite::{Connection, State};
use std::{path::Path, sync::{Arc, Mutex}};

#[derive(Clone)]
pub struct AccountStore {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AccountRecord {
    pub id: i64,
    pub provider: String,
    pub label: String,
    pub status: String,
    pub base_url: String,
    pub model_scope: String,
}

impl AccountStore {
    pub fn open(path: &str) -> Result<Self> {
        let db_path = Path::new(path);
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create db directory {}", parent.display()))?;
        }

        let connection = sqlite::open(path)
            .with_context(|| format!("failed to open sqlite database {}", path))?;

        let store = Self {
            conn: Arc::new(Mutex::new(connection)),
        };

        store.migrate()?;
        store.seed_defaults()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().expect("sqlite mutex poisoned");
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                provider TEXT NOT NULL,
                label TEXT NOT NULL,
                status TEXT NOT NULL,
                base_url TEXT NOT NULL,
                model_scope TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }

    fn seed_defaults(&self) -> Result<()> {
        if !self.list_accounts()?.is_empty() {
            return Ok(());
        }

        self.insert_account("date-now", "date-now primary", "active", "https://date-now.example/v1", "gpt-5.4")?;
        self.insert_account("iflow", "iflow qwen lane", "active", "https://iflow.example/v1", "qwen3-max")?;
        Ok(())
    }

    pub fn list_accounts(&self) -> Result<Vec<AccountRecord>> {
        let conn = self.conn.lock().expect("sqlite mutex poisoned");
        let mut statement = conn.prepare(
            "SELECT id, provider, label, status, base_url, model_scope FROM accounts ORDER BY id ASC",
        )?;

        let mut items = Vec::new();
        while let State::Row = statement.next()? {
            items.push(AccountRecord {
                id: statement.read::<i64, _>(0)?,
                provider: statement.read::<String, _>(1)?,
                label: statement.read::<String, _>(2)?,
                status: statement.read::<String, _>(3)?,
                base_url: statement.read::<String, _>(4)?,
                model_scope: statement.read::<String, _>(5)?,
            });
        }

        Ok(items)
    }

    fn insert_account(
        &self,
        provider: &str,
        label: &str,
        status: &str,
        base_url: &str,
        model_scope: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().expect("sqlite mutex poisoned");
        let sql = format!(
            "INSERT INTO accounts (provider, label, status, base_url, model_scope) VALUES ('{}', '{}', '{}', '{}', '{}')",
            escape_sql(provider),
            escape_sql(label),
            escape_sql(status),
            escape_sql(base_url),
            escape_sql(model_scope)
        );
        conn.execute(sql)?;
        Ok(())
    }
}

fn escape_sql(value: &str) -> String {
    value.replace('"', "\"").replace('\'', "''")
}
