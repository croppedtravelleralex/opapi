use crate::bridge::mapper::{
    map_codex_app_chat_request,
    map_codex_app_response_request,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn proxy_codex_app_chat(
    url: &str,
    timeout_ms: u64,
    model: &str,
    user_text: &str,
    session_namespace: &str,
    session_key_hint: &str,
    freshness_seconds: Option<i64>,
) -> Result<Value, String> {
    let payload = map_codex_app_chat_request(
        model,
        user_text,
        session_namespace,
        session_key_hint,
        freshness_seconds,
    );
    send_bridge_request(url, timeout_ms, payload).await
}

pub async fn proxy_codex_app_response(
    url: &str,
    timeout_ms: u64,
    model: &str,
    input: &str,
    session_namespace: &str,
    session_key_hint: &str,
    freshness_seconds: Option<i64>,
) -> Result<Value, String> {
    let payload = map_codex_app_response_request(
        model,
        input,
        session_namespace,
        session_key_hint,
        freshness_seconds,
    );
    send_bridge_request(url, timeout_ms, payload).await
}

async fn send_bridge_request(
    url: &str,
    timeout_ms: u64,
    payload: Value,
) -> Result<Value, String> {
    let timeout_duration = Duration::from_millis(timeout_ms);
    let payload_text = serde_json::to_string(&payload)
        .map_err(|err| format!("real_ws_payload_serialize_failed:{}", err))?;

    let (mut ws, _) = timeout(timeout_duration, connect_async(url))
        .await
        .map_err(|_| "real_ws_connect_timeout".to_string())?
        .map_err(|err| format!("real_ws_connect_failed:{}", err))?;

    timeout(timeout_duration, ws.send(Message::Text(payload_text.into())))
        .await
        .map_err(|_| "real_ws_send_timeout".to_string())?
        .map_err(|err| format!("real_ws_send_failed:{}", err))?;

    let next_message = timeout(timeout_duration, ws.next())
        .await
        .map_err(|_| "real_ws_receive_timeout".to_string())?
        .ok_or_else(|| "real_ws_connection_closed".to_string())?;

    let message = next_message.map_err(|err| format!("real_ws_receive_failed:{}", err))?;
    parse_ws_message(message)
}

fn parse_ws_message(message: Message) -> Result<Value, String> {
    let payload = match message {
        Message::Text(text) => {
            serde_json::from_str(&text).map_err(|err| format!("real_ws_invalid_json:{}", err))?
        }
        Message::Binary(bytes) => serde_json::from_slice(&bytes)
            .map_err(|err| format!("real_ws_invalid_binary_json:{}", err))?,
        Message::Close(_) => return Err("real_ws_connection_closed".into()),
        other => return Err(format!("real_ws_unexpected_message_type:{:?}", other)),
    };

    normalize_real_payload(payload)
}

fn normalize_real_payload(payload: Value) -> Result<Value, String> {
    if let Some(upstream_payload) = payload.get("upstream_payload") {
        return Ok(upstream_payload.clone());
    }
    if let Some(error) = payload.get("error") {
        return Err(format!("real_ws_upstream_error:{}", error));
    }
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio_tungstenite::tungstenite::Message;

    #[test]
    fn parse_ws_message_accepts_text_json() {
        let parsed = parse_ws_message(Message::Text("{\"ok\":true}".into())).unwrap();
        assert_eq!(parsed, json!({"ok": true}));
    }

    #[test]
    fn parse_ws_message_rejects_ping_frames() {
        let err = parse_ws_message(Message::Ping(vec![1, 2, 3].into())).unwrap_err();
        assert!(err.starts_with("real_ws_unexpected_message_type:"));
    }

    #[test]
    fn normalize_real_payload_unwraps_bridge_response_shape() {
        let parsed = normalize_real_payload(json!({
            "upstream_payload": {
                "runtime": {"session_namespace": "real-ns"},
                "choices": [{"message": {"content": "ok"}}]
            }
        }))
        .unwrap();
        assert_eq!(parsed["runtime"]["session_namespace"], "real-ns");
    }

    #[test]
    fn normalize_real_payload_returns_upstream_error() {
        let err = normalize_real_payload(json!({
            "error": {"code": "bad_upstream", "message": "boom"}
        }))
        .unwrap_err();
        assert!(err.starts_with("real_ws_upstream_error:"));
    }

    #[test]
    fn normalize_real_payload_keeps_nested_runtime_shape_for_followup_extractors() {
        let parsed = normalize_real_payload(json!({
            "upstream_payload": {
                "output": [{
                    "content": [{
                        "handshake": {
                            "session_namespace": "resp-ns",
                            "session_key_hint": "resp-key"
                        },
                        "text": "ok"
                    }]
                }]
            }
        }))
        .unwrap();
        assert_eq!(parsed["output"][0]["content"][0]["handshake"]["session_namespace"], "resp-ns");
        assert_eq!(parsed["output"][0]["content"][0]["handshake"]["session_key_hint"], "resp-key");
    }
}
