# TODO（openclaw-openai-gateway）

## P0：本机轻服务主线
- [x] 收口项目定位：**单机轻服务**，不是大平台
- [x] 保留最小 API 面：`health / models / providers / codex / chat / responses`
- [x] 从主运行面移除 governance 路由
- [ ] 冻结/删除 governance 相关模块与文档
- [ ] 冻结第三方 provider 扩展路径
- [ ] 清理低价值 domain 预留模型
- [ ] 收清仓库边界与工作树污染

## P1：Codex 真实额度主链
- [~] 打通 **Codex App 额度反代链路**（已落采集、admission、入池、bridge 骨架，未接真实浏览器）
- [~] 打通 **Codex Web 额度反代链路**（已落来源入口与 bridge adapter 分派，未接真实采集与真实会话）
- [x] 建立统一 **额度来源抽象**（App / Web）
- [~] 建立统一 **反代执行入口**（已落 source context + session bridge + adapter dispatch）
- [~] 建立最小 **额度池**（已落 admission + pool_members + pool routing）
- [x] 提供最小 **额度观测查询接口**（`/v1/codex/quota-sources` / `/v1/codex/quota-overview`）
- [x] 提供最小 **额度采集写入口**（`/v1/codex/quota/collect`）
- [ ] 让系统能基于真实额度来源对外提供 API Key

## P2：稳定化
- [ ] 做来源探活
- [ ] 做来源失效 / 过期 / fallback
- [ ] 做 pool member 冷却 / 恢复 / 负载更新
- [ ] 补 SQLite repository 真读写测试
- [ ] 补 parser 页面变体兼容测试
- [ ] 补 bridge 失败边界测试
- [ ] 后续按需补轻 scheduler

## 已完成底座
- [x] Rust 工程骨架
- [x] SQLite 控制平面骨架
- [x] `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses`
- [x] Bearer API Key middleware
- [x] request id middleware
- [x] OpenClaw WS client skeleton
- [x] `Codex App / Web` 额度来源骨架
- [x] `Codex` quota overview / collect
- [x] admission skeleton
- [x] pool_members 写回
- [x] pool routing
- [x] source context
- [x] session bridge
- [x] source-aware adapter dispatch
- [x] smoke tests

## 明确不做 / 延后
- [ ] GUI 桌面端
- [ ] 重 dashboard
- [ ] governance 平台化控制面
- [ ] 第三方 provider 市场化扩展
