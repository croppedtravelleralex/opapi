pub struct OpenClawWsClient {
    url: String,
    timeout_ms: u64,
}

impl OpenClawWsClient {
    pub fn new(url: String, timeout_ms: u64) -> Self {
        Self { url, timeout_ms }
    }

    pub async fn check_ready(&self) -> bool {
        let _ = (&self.url, self.timeout_ms);
        true
    }
}
