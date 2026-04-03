use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodexQuotaSourceKind {
    CodexApp,
    CodexWeb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexQuotaSource {
    pub id: String,
    pub kind: CodexQuotaSourceKind,
    pub enabled: bool,
    pub display_name: String,
    pub provider_id: String,
    pub base_url: Option<String>,
    pub observation_path: String,
    pub notes: Option<String>,
}

pub fn default_codex_quota_sources() -> Vec<CodexQuotaSource> {
    vec![
        CodexQuotaSource {
            id: "codex-app".into(),
            kind: CodexQuotaSourceKind::CodexApp,
            enabled: true,
            display_name: "Codex App".into(),
            provider_id: "codex.app".into(),
            base_url: Some("https://chatgpt.com".into()),
            observation_path: "/codex".into(),
            notes: Some("桌面 App / 内嵌 WebView 会话额度观测入口".into()),
        },
        CodexQuotaSource {
            id: "codex-web".into(),
            kind: CodexQuotaSourceKind::CodexWeb,
            enabled: true,
            display_name: "Codex Web".into(),
            provider_id: "codex.web".into(),
            base_url: Some("https://chatgpt.com".into()),
            observation_path: "/codex".into(),
            notes: Some("浏览器网页会话额度观测入口".into()),
        },
    ]
}
