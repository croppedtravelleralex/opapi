use crate::{
    bridge::mapper::{map_chat_response, map_response_output},
    codex::pool_router::RoutedPoolMember,
};
use serde_json::Value;

#[derive(Clone)]
pub struct CodexExecutor;

impl CodexExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_chat(
        &self,
        member: &RoutedPoolMember,
        model: &str,
        user_text: &str,
    ) -> Result<Value, String> {
        Ok(map_chat_response(
            model,
            &format!("codex routed via {} [{}]: {}", member.child_account_id, member.admission_level, user_text),
        ))
    }

    pub async fn execute_response(
        &self,
        member: &RoutedPoolMember,
        model: &str,
        input: &str,
    ) -> Result<Value, String> {
        Ok(map_response_output(
            model,
            &format!("codex routed via {} [{}]: {}", member.child_account_id, member.admission_level, input),
        ))
    }
}
