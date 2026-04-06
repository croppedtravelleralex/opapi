#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

cleanup() {
  pkill -f 'scripts/mock_upstream.py 19091 mock-a' >/dev/null 2>&1 || true
  pkill -f 'scripts/mock_upstream.py 19092 mock-b' >/dev/null 2>&1 || true
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

python3 scripts/mock_upstream.py 19091 mock-a >/tmp/mock-a.log 2>&1 &
python3 scripts/mock_upstream.py 19092 mock-b >/tmp/mock-b.log 2>&1 &

PORT=18088 \
GATEWAY_API_KEYS=sk-local-demo \
UPSTREAMS='a|http://127.0.0.1:19091|dummy|append-v1;b|http://127.0.0.1:19092|dummy' \
MODEL_UPSTREAM_MAP='gpt-5.4=a,qwen3-max=b' \
cargo run >/tmp/sub2api-smoke.log 2>&1 &
SERVER_PID=$!

for _ in {1..30}; do
  if curl -fsS http://127.0.0.1:18088/healthz >/tmp/sub2api-health.out 2>/dev/null; then
    break
  fi
  sleep 1
done

curl -fsS http://127.0.0.1:18088/healthz >/tmp/sub2api-health.out

NOAUTH_CODE=$(curl -s -o /tmp/sub2api-models-noauth.out -w '%{http_code}' http://127.0.0.1:18088/v1/models)
AUTH_CODE=$(curl -s -o /tmp/sub2api-models-auth.out -w '%{http_code}' http://127.0.0.1:18088/v1/models -H 'Authorization: Bearer sk-local-demo')

curl -fsS -X POST http://127.0.0.1:18088/v1/chat/completions \
  -H 'Authorization: Bearer sk-local-demo' \
  -H 'Content-Type: application/json' \
  -d '{"model":"gpt-5.4","messages":[{"role":"user","content":"hello-a"}]}' >/tmp/sub2api-chat-a.out

curl -fsS -X POST http://127.0.0.1:18088/v1/chat/completions \
  -H 'Authorization: Bearer sk-local-demo' \
  -H 'Content-Type: application/json' \
  -d '{"model":"qwen3-max","messages":[{"role":"user","content":"hello-b"}]}' >/tmp/sub2api-chat-b.out

python3 - <<'PY'
import json
from pathlib import Path

health = json.loads(Path('/tmp/sub2api-health.out').read_text())
models_auth = json.loads(Path('/tmp/sub2api-models-auth.out').read_text())
chat_a = json.loads(Path('/tmp/sub2api-chat-a.out').read_text())
chat_b = json.loads(Path('/tmp/sub2api-chat-b.out').read_text())
noauth_code = Path('/tmp/sub2api-models-noauth.out').read_text() if Path('/tmp/sub2api-models-noauth.out').exists() else ''

def assert_true(cond, message):
    if not cond:
        raise SystemExit(message)

assert_true(health.get('ok') is True, 'healthz did not return ok=true')
assert_true(len(models_auth.get('data', [])) >= 2, 'models list is unexpectedly short')
assert_true(chat_a.get('id') == 'chatcmpl-mock-a', 'gpt-5.4 did not route to mock-a')
assert_true(chat_b.get('id') == 'chatcmpl-mock-b', 'qwen3-max did not route to mock-b')
PY

if [[ "$NOAUTH_CODE" != "401" ]]; then
  echo "expected /v1/models without auth to return 401, got $NOAUTH_CODE" >&2
  exit 1
fi

if [[ "$AUTH_CODE" != "200" ]]; then
  echo "expected /v1/models with auth to return 200, got $AUTH_CODE" >&2
  exit 1
fi

echo "smoke ok"
