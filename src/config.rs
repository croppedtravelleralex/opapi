use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_title: String,
    pub default_models: Vec<String>,
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

        Ok(Self {
            host,
            port,
            api_title,
            default_models,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
