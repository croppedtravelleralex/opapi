#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${1:-http://127.0.0.1:8088}"
TOKEN="${2:-sk-local-demo}"

printf '== healthz ==\n'
curl -sS "$BASE_URL/healthz"
printf '\n\n== models ==\n'
curl -sS "$BASE_URL/v1/models" -H "Authorization: Bearer $TOKEN"
printf '\n\n== chat ==\n'
curl -sS -X POST "$BASE_URL/v1/chat/completions" \
  -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{"model":"gpt-5.4","messages":[{"role":"user","content":"hello"}]}'
printf '\n'
