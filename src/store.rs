use anyhow::{Context, Result};
use serde::Serialize;
use sqlite::{Connection, State};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

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
    pub email: String,
    pub password: String,
    pub user_id: String,
    pub region: String,
    pub token: String,
    pub refresh_token: String,
    pub trial_end_time: i64,
    pub cashier_url: String,
    pub extra_json: String,
}

#[derive(Debug, Clone)]
pub struct ImportedAccount {
    pub provider: String,
    pub label: String,
    pub status: String,
    pub base_url: String,
    pub model_scope: String,
    pub email: String,
    pub password: String,
    pub user_id: String,
    pub region: String,
    pub token: String,
    pub refresh_token: String,
    pub trial_end_time: i64,
    pub cashier_url: String,
    pub extra_json: String,
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
                model_scope TEXT NOT NULL,
                email TEXT NOT NULL DEFAULT '',
                password TEXT NOT NULL DEFAULT '',
                user_id TEXT NOT NULL DEFAULT '',
                region TEXT NOT NULL DEFAULT '',
                token TEXT NOT NULL DEFAULT '',
                refresh_token TEXT NOT NULL DEFAULT '',
                trial_end_time INTEGER NOT NULL DEFAULT 0,
                cashier_url TEXT NOT NULL DEFAULT '',
                extra_json TEXT NOT NULL DEFAULT '{}'
            );
            "#,
        )?;
        self.ensure_column(&conn, "extra_json", "TEXT NOT NULL DEFAULT '{}' ")?;
        conn.execute("DROP INDEX IF EXISTS idx_accounts_provider_email")?;
        conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_provider_email ON accounts(provider, email);")?;
        Ok(())
    }

    fn ensure_column(&self, conn: &Connection, name: &str, definition: &str) -> Result<()> {
        let mut statement = conn.prepare("PRAGMA table_info(accounts)")?;
        while let State::Row = statement.next()? {
            let existing: String = statement.read::<String, _>(1)?;
            if existing == name {
                return Ok(());
            }
        }
        conn.execute(format!("ALTER TABLE accounts ADD COLUMN {} {}", name, definition))?;
        Ok(())
    }

    fn seed_defaults(&self) -> Result<()> {
        if !self.list_accounts()?.is_empty() {
            return Ok(());
        }

        self.upsert_imported_account(ImportedAccount {
            provider: "date-now".to_string(),
            label: "date-now primary".to_string(),
            status: "active".to_string(),
            base_url: "https://date-now.example/v1".to_string(),
            model_scope: "gpt-5.4".to_string(),
            email: "demo-date-now@example.com".to_string(),
            password: "demo-password".to_string(),
            user_id: "".to_string(),
            region: "".to_string(),
            token: "".to_string(),
            refresh_token: "".to_string(),
            trial_end_time: 0,
            cashier_url: "".to_string(),
            extra_json: "{}".to_string(),
        })?;
        self.upsert_imported_account(ImportedAccount {
            provider: "iflow".to_string(),
            label: "iflow qwen lane".to_string(),
            status: "active".to_string(),
            base_url: "https://iflow.example/v1".to_string(),
            model_scope: "qwen3-max".to_string(),
            email: "demo-iflow@example.com".to_string(),
            password: "demo-password".to_string(),
            user_id: "".to_string(),
            region: "".to_string(),
            token: "".to_string(),
            refresh_token: "".to_string(),
            trial_end_time: 0,
            cashier_url: "".to_string(),
            extra_json: "{}".to_string(),
        })?;
        Ok(())
    }

    pub fn list_accounts(&self) -> Result<Vec<AccountRecord>> {
        let conn = self.conn.lock().expect("sqlite mutex poisoned");
        let mut statement = conn.prepare(
            "SELECT id, provider, label, status, base_url, model_scope, email, password, user_id, region, token, refresh_token, trial_end_time, cashier_url, extra_json FROM accounts ORDER BY id ASC",
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
                email: statement.read::<String, _>(6)?,
                password: statement.read::<String, _>(7)?,
                user_id: statement.read::<String, _>(8)?,
                region: statement.read::<String, _>(9)?,
                token: statement.read::<String, _>(10)?,
                refresh_token: statement.read::<String, _>(11)?,
                trial_end_time: statement.read::<i64, _>(12)?,
                cashier_url: statement.read::<String, _>(13)?,
                extra_json: statement.read::<String, _>(14)?,
            });
        }

        Ok(items)
    }

    pub fn upsert_imported_account(&self, account: ImportedAccount) -> Result<()> {
        let conn = self.conn.lock().expect("sqlite mutex poisoned");
        let mut statement = conn.prepare("SELECT id FROM accounts WHERE provider = ? AND email = ? LIMIT 1")?;
        statement.bind((1, account.provider.as_str()))?;
        statement.bind((2, account.email.as_str()))?;

        let existing_id = match statement.next()? {
            State::Row => Some(statement.read::<i64, _>(0)?),
            State::Done => None,
        };
        drop(statement);

        if let Some(id) = existing_id {
            let sql = format!(
                "UPDATE accounts SET label='{}', status='{}', base_url='{}', model_scope='{}', password='{}', user_id='{}', region='{}', token='{}', refresh_token='{}', trial_end_time={}, cashier_url='{}', extra_json='{}' WHERE id={}",
                escape_sql(&account.label),
                escape_sql(&account.status),
                escape_sql(&account.base_url),
                escape_sql(&account.model_scope),
                escape_sql(&account.password),
                escape_sql(&account.user_id),
                escape_sql(&account.region),
                escape_sql(&account.token),
                escape_sql(&account.refresh_token),
                account.trial_end_time,
                escape_sql(&account.cashier_url),
                escape_sql(&account.extra_json),
                id
            );
            conn.execute(sql)?;
        } else {
            let sql = format!(
                "INSERT INTO accounts (provider, label, status, base_url, model_scope, email, password, user_id, region, token, refresh_token, trial_end_time, cashier_url, extra_json) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')",
                escape_sql(&account.provider),
                escape_sql(&account.label),
                escape_sql(&account.status),
                escape_sql(&account.base_url),
                escape_sql(&account.model_scope),
                escape_sql(&account.email),
                escape_sql(&account.password),
                escape_sql(&account.user_id),
                escape_sql(&account.region),
                escape_sql(&account.token),
                escape_sql(&account.refresh_token),
                account.trial_end_time,
                escape_sql(&account.cashier_url),
                escape_sql(&account.extra_json),
            );
            conn.execute(sql)?;
        }

        Ok(())
    }
}

fn escape_sql(value: &str) -> String {
    value.replace('"', "\"").replace('\'', "''")
}
