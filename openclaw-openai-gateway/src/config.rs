use crate::bridge::client::OpenClawWsTransportMode;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub openclaw_ws_url: String,
    pub openclaw_api_timeout_ms: u64,
    pub openclaw_ws_transport_mode: OpenClawWsTransportMode,
    pub api_keys: Vec<String>,
    pub models: Vec<String>,
    pub sqlite_path: String,
    pub codex_session_bridge_mode: String,
    #[allow(dead_code)]
    pub third_party_provider_id: Option<String>,
    #[allow(dead_code)]
    pub third_party_base_url: Option<String>,
    #[allow(dead_code)]
    pub third_party_api_key: Option<String>,
    #[allow(dead_code)]
    pub third_party_model: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let app_port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| "invalid APP_PORT")?;

        let openclaw_ws_url =
            env::var("OPENCLAW_WS_URL").unwrap_or_else(|_| "ws://127.0.0.1:18789".into());

        let openclaw_api_timeout_ms = env::var("OPENCLAW_API_TIMEOUT_MS")
            .unwrap_or_else(|_| "15000".into())
            .parse()
            .map_err(|_| "invalid OPENCLAW_API_TIMEOUT_MS")?;

        let api_keys = env::var("API_KEYS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        let models = env::var("MODELS")
            .unwrap_or_else(|_| "openclaw-default".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        let sqlite_path = env::var("SQLITE_PATH")
            .unwrap_or_else(|_| "dev.sqlite3".into());

        Ok(Self {
            app_host,
            app_port,
            openclaw_ws_url,
            openclaw_api_timeout_ms,
            openclaw_ws_transport_mode: OpenClawWsTransportMode::from_str(
                &env::var("OPENCLAW_WS_TRANSPORT_MODE").unwrap_or_else(|_| "mock".into()),
            ),
            api_keys,
            models,
            sqlite_path,
            codex_session_bridge_mode: env::var("CODEX_SESSION_BRIDGE_MODE").unwrap_or_else(|_| "mock".into()),
            third_party_provider_id: env::var("THIRD_PARTY_PROVIDER_ID").ok(),
            third_party_base_url: env::var("THIRD_PARTY_BASE_URL").ok(),
            third_party_api_key: env::var("THIRD_PARTY_API_KEY").ok(),
            third_party_model: env::var("THIRD_PARTY_MODEL").ok(),
        })
    }
}
