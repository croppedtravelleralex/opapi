use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

#[derive(Clone, Debug)]
pub struct UpstreamConfig {
    pub base_url: String,
    pub api_key: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_title: String,
    pub default_models: Vec<String>,
    pub upstream_base_url: Option<String>,
    pub upstream_api_key: Option<String>,
    pub gateway_api_keys: Vec<String>,
    pub gateway_api_keys_file: Option<String>,
    pub upstreams: HashMap<String, UpstreamConfig>,
    pub model_upstream_map: HashMap<String, String>,
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
        let gateway_api_keys_file = std::env::var("GATEWAY_API_KEYS_FILE")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());

        let mut gateway_api_keys: Vec<String> = std::env::var("GATEWAY_API_KEYS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(file) = &gateway_api_keys_file {
            gateway_api_keys.extend(load_api_keys_from_file(file)?);
            gateway_api_keys.sort();
            gateway_api_keys.dedup();
        }

        let upstreams = parse_upstreams(std::env::var("UPSTREAMS").unwrap_or_default());
        let model_upstream_map = parse_model_upstream_map(std::env::var("MODEL_UPSTREAM_MAP").unwrap_or_default());

        Ok(Self {
            host,
            port,
            api_title,
            default_models,
            upstream_base_url,
            upstream_api_key,
            gateway_api_keys,
            gateway_api_keys_file,
            upstreams,
            model_upstream_map,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn upstream_for_model(&self, model: &str) -> Option<UpstreamConfig> {
        if let Some(upstream_name) = self.model_upstream_map.get(model) {
            return self.upstreams.get(upstream_name).cloned();
        }

        match (&self.upstream_base_url, &self.upstream_api_key) {
            (Some(base_url), Some(api_key)) => Some(UpstreamConfig {
                base_url: base_url.clone(),
                api_key: api_key.clone(),
            }),
            _ => None,
        }
    }
}

fn load_api_keys_from_file(path: &str) -> Result<Vec<String>> {
    let path = Path::new(path);
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect())
}

fn parse_upstreams(raw: String) -> HashMap<String, UpstreamConfig> {
    let mut map = HashMap::new();

    for entry in raw.split(';').map(|v| v.trim()).filter(|v| !v.is_empty()) {
        let mut parts = entry.split('|').map(|v| v.trim());
        let name = parts.next().unwrap_or_default();
        let base_url = parts.next().unwrap_or_default().trim_end_matches('/');
        let api_key = parts.next().unwrap_or_default();

        if !name.is_empty() && !base_url.is_empty() && !api_key.is_empty() {
            map.insert(
                name.to_string(),
                UpstreamConfig {
                    base_url: base_url.to_string(),
                    api_key: api_key.to_string(),
                },
            );
        }
    }

    map
}

fn parse_model_upstream_map(raw: String) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for entry in raw.split(',').map(|v| v.trim()).filter(|v| !v.is_empty()) {
        if let Some((model, upstream)) = entry.split_once('=') {
            let model = model.trim();
            let upstream = upstream.trim();
            if !model.is_empty() && !upstream.is_empty() {
                map.insert(model.to_string(), upstream.to_string());
            }
        }
    }

    map
}
