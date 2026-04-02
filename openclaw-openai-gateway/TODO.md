# TODO（openclaw-openai-gateway）

## P0
- [x] 建立 Rust 工程骨架
- [x] 建立 `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses`
- [x] 接入 Bearer API Key 中间件
- [x] 接入 request id 中间件
- [x] 建立 OpenClaw WS client skeleton
- [x] 落下第一批 SQLite migrations
- [x] 让工程 `cargo check` 通过
- [x] 建立项目内 `CURRENT_DIRECTION.md`
- [x] 建立项目内 `CURRENT_TASK.md`
- [x] 建立项目内 `STATUS.md`
- [x] 建立项目内 `TODO.md`
- [x] 建立 `domain / routing / providers / governance` 主梁目录
- [x] 把 fake `check_ready()` 改成真实上游探测
- [x] 把 fake `chat` 改成最小真实 bridge
- [x] 把 fake `responses` 改成最小真实 bridge
- [x] 落下 Explainability v1
- [x] 落下 Audit skeleton
- [x] 落下 repository / in-memory persistence skeleton
- [x] 接入 SQLite skeleton
- [x] `/v1/models` 切到 SQLite-backed reads
- [x] 补 smoke tests
- [x] 补 `FINAL_FEATURE_MAP.md`
- [x] 把 `providers` 主读源切到 SQLite-backed reads
- [x] 把 `provider_capabilities / model_availability` 真落表并 seed
- [x] 让 routing 决策使用 SQLite provider 数据
- [x] 让 capability / availability 开始参与 routing 决策
- [x] 把 audit 持久化到 SQLite

## P1
- [ ] 补 auth / health / models / chat / responses 更完整测试
- [ ] 补 SQLite repository 真读写测试
- [ ] 把 audit 持久化到 SQLite
- [ ] 建立 Account / AccountPool 最小骨架
- [ ] 建立 governance / config snapshot / release record / change plan 最小骨架

## P2
- [ ] 支持 SSE 流式
- [ ] 建立真正多 Provider 行为
- [ ] 建立 capability / availability 真正参与路由
- [ ] 建立 SessionPool / WebSessionPool
- [ ] 建立 automation / maintenance job 主梁
- [ ] 扩展 embeddings / images / audio 能力面
