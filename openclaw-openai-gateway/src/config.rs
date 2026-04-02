use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub openclaw_ws_url: String,
    pub openclaw_api_timeout_ms: u64,
    pub api_keys: Vec<String>,
    pub models: Vec<String>,
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

        Ok(Self {
            app_host,
            app_port,
            openclaw_ws_url,
            openclaw_api_timeout_ms,
            api_keys,
            models,
        })
    }
}
