# TODO（openclaw-openai-gateway）

## P0
- [x] 建立 Rust 工程骨架
- [x] 建立 `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses`
- [x] 接入 Bearer API Key 中间件
- [x] 接入 request id 中间件
- [x] 建立 OpenClaw WS client skeleton
- [x] 落下第一批 SQLite migrations
- [x] 让工程 `cargo check` 通过
- [ ] 建立项目内 `CURRENT_DIRECTION.md`
- [ ] 建立项目内 `CURRENT_TASK.md`
- [ ] 建立项目内 `STATUS.md`
- [ ] 建立项目内 `TODO.md`
- [ ] 把工程目录扩到六根主梁（`providers / routing / governance / ops / automation / domain`）
- [ ] 把 fake `check_ready()` 改成真实上游探测
- [ ] 把 fake `chat` 改成最小真实 bridge
- [ ] 把 fake `responses` 改成最小真实 bridge

## P1
- [ ] 补 auth / health / models / chat / responses 测试
- [ ] 补 Docker Compose / systemd 启动方式
- [ ] 补 explainability 最小记录
- [ ] 补 usage / latency 最小记录

## P2
- [ ] 建立 ModelCatalog / ProviderPool 最小骨架
- [ ] 建立 RoutingPolicy / RoutingDecision 最小骨架
- [ ] 建立 Account / AccountPool 最小骨架
- [ ] 建立 governance / config snapshot 最小骨架
