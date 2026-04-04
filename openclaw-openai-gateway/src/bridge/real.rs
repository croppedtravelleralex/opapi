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
    match message {
        Message::Text(text) => {
            serde_json::from_str(&text).map_err(|err| format!("real_ws_invalid_json:{}", err))
        }
        Message::Binary(bytes) => serde_json::from_slice(&bytes)
            .map_err(|err| format!("real_ws_invalid_binary_json:{}", err)),
        Message::Close(_) => Err("real_ws_connection_closed".into()),
        other => Err(format!("real_ws_unexpected_message_type:{:?}", other)),
    }
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
}
