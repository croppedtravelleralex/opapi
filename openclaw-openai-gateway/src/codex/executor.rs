use crate::codex::{
    pool_router::RoutedPoolMember,
    session_bridge::CodexSessionBridge,
    source_context::SourceContextRepository,
};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct CodexExecutor {
    dsn: String,
    session_bridge_mode: String,
}

impl CodexExecutor {
    pub fn new(dsn: String, session_bridge_mode: String) -> Self {
        Self { dsn, session_bridge_mode }
    }

    pub async fn execute_chat(
        &self,
        member: &RoutedPoolMember,
        model: &str,
        user_text: &str,
    ) -> Result<Value, String> {
        let ctx = SourceContextRepository::new(self.dsn.clone())
            .latest_for_child(&member.child_account_id)?;
        let bridge = CodexSessionBridge::new(self.session_bridge_mode.clone());
        let bridged = bridge
            .run_chat(&ctx.source_id, &ctx.source_page, model, user_text)
            .await?;
        Ok(json!({
            "id": format!("chatcmpl-codex-{}", chrono::Utc::now().timestamp_millis()),
            "object": "chat.completion",
            "created": chrono::Utc::now().timestamp(),
            "model": model,
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": format!(
                        "codex routed via {} [{}] source={} page={}: {}",
                        member.child_account_id,
                        member.admission_level,
                        ctx.source_id,
                        ctx.source_page,
                        bridged
                    )
                },
                "finish_reason": "stop"
            }]
        }))
    }

    pub async fn execute_response(
        &self,
        member: &RoutedPoolMember,
        model: &str,
        input: &str,
    ) -> Result<Value, String> {
        let ctx = SourceContextRepository::new(self.dsn.clone())
            .latest_for_child(&member.child_account_id)?;
        let bridge = CodexSessionBridge::new(self.session_bridge_mode.clone());
        let bridged = bridge
            .run_response(&ctx.source_id, &ctx.source_page, model, input)
            .await?;
        Ok(json!({
            "id": format!("resp-codex-{}", chrono::Utc::now().timestamp_millis()),
            "object": "response",
            "created_at": chrono::Utc::now().timestamp(),
            "model": model,
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{
                    "type": "output_text",
                    "text": format!(
                        "codex routed via {} [{}] source={} page={}: {}",
                        member.child_account_id,
                        member.admission_level,
                        ctx.source_id,
                        ctx.source_page,
                        bridged
                    )
                }]
            }]
        }))
    }
}
