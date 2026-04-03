use serde_json::{json, Value};

pub fn map_chat_request(model: &str, user_text: &str) -> Value {
    json!({
        "type": "chat.completion.create",
        "model": model,
        "input": {
            "messages": [
                {
                    "role": "user",
                    "content": user_text
                }
            ]
        }
    })
}

pub fn map_codex_app_chat_request(
    model: &str,
    user_text: &str,
    session_namespace: &str,
    session_key_hint: &str,
    freshness_seconds: Option<i64>,
) -> Value {
    json!({
        "type": "codex.app.chat.create",
        "model": model,
        "source": "codex-app",
        "handshake": {
            "session_namespace": session_namespace,
            "session_key_hint": session_key_hint,
            "freshness_seconds": freshness_seconds
        },
        "input": {
            "messages": [
                {
                    "role": "user",
                    "content": user_text
                }
            ]
        }
    })
}

pub fn map_chat_response(model: &str, assistant_text: &str) -> Value {
    json!({
        "id": format!("chatcmpl-proxy-{}", chrono::Utc::now().timestamp_millis()),
        "object": "chat.completion",
        "created": chrono::Utc::now().timestamp(),
        "model": model,
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": assistant_text
            },
            "finish_reason": "stop"
        }]
    })
}

pub fn map_response_request(model: &str, input: &str) -> Value {
    json!({
        "type": "response.create",
        "model": model,
        "input": input
    })
}

pub fn map_codex_app_response_request(
    model: &str,
    input: &str,
    session_namespace: &str,
    session_key_hint: &str,
    freshness_seconds: Option<i64>,
) -> Value {
    json!({
        "type": "codex.app.response.create",
        "model": model,
        "source": "codex-app",
        "handshake": {
            "session_namespace": session_namespace,
            "session_key_hint": session_key_hint,
            "freshness_seconds": freshness_seconds
        },
        "input": input
    })
}

pub fn map_response_output(model: &str, output_text: &str) -> Value {
    json!({
        "id": format!("resp-proxy-{}", chrono::Utc::now().timestamp_millis()),
        "object": "response",
        "created_at": chrono::Utc::now().timestamp(),
        "model": model,
        "output": [{
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "output_text",
                "text": output_text
            }]
        }]
    })
}
