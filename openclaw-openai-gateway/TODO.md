# TODO（openclaw-openai-gateway）

## P0：核心主线（先反代出额度）
- [x] 打通主线设计文档并写入项目
- [x] 建立母号 / 子号 / 空间成员 / 邀请任务 / 额度快照 / 池成员 / 外部 API Key 数据模型
- [x] 将 `parent_accounts / child_accounts / space_memberships / invite_tasks / quota_snapshots / pool_members / proxy_api_keys` 加入 SQLite schema
- [~] 打通 **Codex App 额度反代链路**（已落最小采集器骨架，未接真实浏览器）
- [ ] 打通 **Web 会话 / Web 额度反代链路**
- [x] 建立统一 **额度来源抽象**（App / Web）
- [ ] 建立统一 **反代执行入口**（chat / responses 真分发）
- [ ] 让系统能基于真实额度来源对外提供 API Key
- [ ] 建立最小 **额度池**（可用/不可用/来源类型）
- [x] 提供最小 **额度观测查询接口**（`/v1/codex/quota-sources` / `/v1/codex/quota-overview`）
- [x] 提供最小 **额度采集写入口**（`/v1/codex/quota/collect`）

## P1：反代稳定化
- [ ] 做额度来源探活
- [ ] 做额度可用性状态记录
- [ ] 做基础失败切换（同类来源内切换）
- [ ] 做基础审计查询接口
- [ ] 补 auth / health / models / chat / responses 更完整测试
- [ ] 补 SQLite repository 真读写测试
- [ ] 补 parser 页面变体兼容测试

## P2：池化与治理增强
- [ ] 建立 Account / AccountPool 最小骨架
- [ ] 建立容灾机制
- [ ] 建立平衡/调度机制
- [ ] 建立 SessionPool / WebSessionPool
- [ ] 建立 governance / config snapshot / release record / change plan 更完整工作流
- [ ] 支持 SSE 流式
- [ ] 扩展 embeddings / images / audio 能力面
- [ ] 建立 automation / maintenance job 主梁

## 已完成的底座
- [x] Rust 工程骨架
- [x] `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses`
- [x] Bearer API Key 中间件
- [x] request id 中间件
- [x] OpenClaw WS client skeleton
- [x] SQLite skeleton 与最小控制平面落库
- [x] `/v1/models` / `/v1/providers` SQLite-backed reads
- [x] `provider_capabilities / model_availability` 建表与 seed
- [x] routing 读取 SQLite provider 数据
- [x] capability / availability 开始参与 routing
- [x] audit 持久化到 SQLite
- [x] governance skeleton
- [x] smoke tests
- [x] `Codex App / Web` 额度来源骨架
- [x] `Codex` 额度观测汇总接口
- [x] `Codex App` 最小额度采集器骨架

## 暂缓 / 降级优先级
- [ ] 第三方 API Key + BaseURL provider 扩展（暂不作为当前主线）
