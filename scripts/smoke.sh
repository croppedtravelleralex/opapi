#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${1:-http://127.0.0.1:8088}"
TOKEN="${2:-sk-local-demo}"
MODEL_A="${3:-gpt-5.4}"
MODEL_B="${4:-qwen3-max}"

printf '== healthz ==\n'
curl -sS "$BASE_URL/healthz"

printf '\n\n== models ==\n'
curl -sS "$BASE_URL/v1/models" -H "Authorization: Bearer $TOKEN"

printf '\n\n== chat model A ==\n'
curl -sS -X POST "$BASE_URL/v1/chat/completions" \
  -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' \
  -d "{\"model\":\"$MODEL_A\",\"messages\":[{\"role\":\"user\",\"content\":\"hello from model A\"}]}"

printf '\n\n== chat model B ==\n'
curl -sS -X POST "$BASE_URL/v1/chat/completions" \
  -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' \
  -d "{\"model\":\"$MODEL_B\",\"messages\":[{\"role\":\"user\",\"content\":\"hello from model B\"}]}"

printf '\n'
