# openclaw-openai-gateway

V1 engineering skeleton for an OpenAI-compatible gateway in front of OpenClaw.

## Current status
This folder already contains:
- Rust project skeleton
- basic config loading
- `healthz`
- `readyz`
- `/v1/models`
- Bearer API key middleware
- request id middleware
- OpenClaw WS client skeleton
- first batch SQLite migrations

## Immediate next steps
1. Make the project compile and run cleanly
2. Add domain structs and repository skeletons
3. Replace the fake `check_ready()` with a real WS readiness probe
4. Implement `/v1/chat/completions` and `/v1/responses` minimal non-stream handlers
5. Add tests for auth, health, models

## Run (planned)
```bash
cp .env.example .env
cargo run
```
