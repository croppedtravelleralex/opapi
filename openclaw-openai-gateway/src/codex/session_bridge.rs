#[derive(Clone)]
pub struct CodexSessionBridge {
    mode: String,
}

impl CodexSessionBridge {
    pub fn new(mode: String) -> Self {
        Self { mode }
    }

    pub async fn run_chat(
        &self,
        source_id: &str,
        source_page: &str,
        _model: &str,
        user_text: &str,
    ) -> Result<String, String> {
        match self.mode.as_str() {
            "mock" => Ok(format!("mock-session-bridge source={} page={} input={}", source_id, source_page, user_text)),
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
    }

    pub async fn run_response(
        &self,
        source_id: &str,
        source_page: &str,
        _model: &str,
        input: &str,
    ) -> Result<String, String> {
        match self.mode.as_str() {
            "mock" => Ok(format!("mock-session-bridge source={} page={} input={}", source_id, source_page, input)),
            other => Err(format!("unsupported_codex_session_bridge_mode:{}", other)),
        }
    }
}
