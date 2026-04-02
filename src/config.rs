use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_title: String,
    pub default_models: Vec<String>,
    pub upstream_base_url: Option<String>,
    pub upstream_api_key: Option<String>,
    pub gateway_api_keys: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8088);
        let api_title = std::env::var("API_TITLE").unwrap_or_else(|_| "sub2api-gateway".to_string());
        let default_models = std::env::var("DEFAULT_MODELS")
            .unwrap_or_else(|_| "gpt-5.4,gpt-4.1-mini,qwen3-max".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let upstream_base_url = std::env::var("UPSTREAM_BASE_URL")
            .ok()
            .map(|v| v.trim().trim_end_matches('/').to_string())
            .filter(|v| !v.is_empty());
        let upstream_api_key = std::env::var("UPSTREAM_API_KEY")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let gateway_api_keys = std::env::var("GATEWAY_API_KEYS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            host,
            port,
            api_title,
            default_models,
            upstream_base_url,
            upstream_api_key,
            gateway_api_keys,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
