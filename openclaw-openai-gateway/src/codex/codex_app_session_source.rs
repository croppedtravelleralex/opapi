use crate::codex::codex_app_adapter::{CodexAppHandshakeMeta, CodexAppRequestContext};
use std::env;

#[derive(Clone, Debug)]
pub struct CodexAppSessionSource {
    namespace_prefix: String,
    key_salt: String,
}

impl CodexAppSessionSource {
    pub fn from_env() -> Self {
        Self {
            namespace_prefix: env::var("CODEX_APP_SESSION_NAMESPACE_PREFIX")
                .unwrap_or_else(|_| "codex-app".to_string()),
            key_salt: env::var("CODEX_APP_SESSION_KEY_SALT")
                .unwrap_or_else(|_| "local-dev".to_string()),
        }
    }

    pub fn resolve(&self, ctx: &CodexAppRequestContext) -> CodexAppHandshakeMeta {
        let freshness_seconds = chrono::DateTime::parse_from_rfc3339(&ctx.observed_at)
            .ok()
            .map(|dt| chrono::Utc::now().signed_duration_since(dt.with_timezone(&chrono::Utc)).num_seconds())
            .filter(|v| *v >= 0);

        CodexAppHandshakeMeta {
            session_namespace: format!("{}:{}", self.namespace_prefix, ctx.child_account_id),
            session_key_hint: format!(
                "{}:{}:{}:{}",
                self.key_salt, ctx.child_account_id, ctx.source_id, ctx.source_page
            ),
            freshness_seconds,
        }
    }
}
