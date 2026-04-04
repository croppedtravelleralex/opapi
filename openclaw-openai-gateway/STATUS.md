# 当前状态（openclaw-openai-gateway）

## 结论
当前工程的**旧主线已经废弃**：不再优先扩第三方 provider。

当前新主线是：

**从 Codex App / Web 网页端反代出真实额度 → 形成额度池 → 对外分配独立 API Key。**

当前阶段判断：
- **骨架工程进度：86%–91%**
- **最终功能进度：28%–38%**

说明：
当前底座已经比较完整，但真正核心的“额度反代主链路”还没开始落代码，所以最终功能进度仍然不高。

---

## 已完成
- Rust 工程骨架已稳定
- `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses` 已落下
- Bearer API Key 中间件已接入
- request id 中间件已接入
- 最小 `Gateway Provider` bridge 已跑通
- `RoutingPolicy / RoutingDecision` 最小骨架已落下
- Explainability v1 已落下（`x-routing-explain`）
- Audit skeleton 已落下（`x-audit-action`）
- SQLite 已真实接入
- `/v1/models` / `/v1/providers` 已切到 SQLite-backed reads
- `provider_capabilities / model_availability` 已可建表并 seed
- audit 已落 SQLite（`audit_events`）
- Codex quota 采集 / admission / pool member 持久化主链已跑通
- Codex App session source 已支持 runtime / sqlite / default 多层来源
- OpenClaw WS transport 已拆成 mock / real skeleton，并有 real ws roundtrip compatibility layer
- 自动注册机 v1 已落下：
  - `auto-register`
  - `dispatch`
  - `worker/run`
  - `dead-letter/recover`
- 注册状态机已具备：
  - `register-account`
  - `verify-email`
  - `verify-invite`
  - `accept-invite`
  - `collect-quota`
  - `warmup-pool`
- 安全控制已具备：retry / backoff / lease / risk_score / dead-letter / recover
- 邮箱池已具备：
  - bulk import
  - poll
  - mailbox bindings
  - poll runs
  - quality_score
  - expansion_tier
  - reservation_count
  - capacity events
  - overview / expand / tiering
- 自动化目标已具备：discover / try / attempt history / suggestion
- 轻运维面板 / 调度入口已具备：
  - `GET /ops/overview`
  - `GET /ops/dashboard`
  - `POST /ops/scheduler/tick`
- dashboard 定位已明确：**只走 SSH 远程转发，不直接公网暴露**
- smoke tests 已扩到通过（`17 + 17 + 34`）
- 已完成新主线设计文档：
  - `VISION_QUOTA_PROXY.md`
  - `DESIGN_PARENT_CHILD_SPACE_MODEL.md`
  - `DESIGN_QUOTA_OBSERVATION.md`
  - `DESIGN_POOL_ADMISSION_AND_EJECTION.md`
  - `DESIGN_FINGERPRINT_BROWSER_ADAPTER.md`
  - `DESIGN_PROXY_KEY_GATEWAY.md`
  - `RUNTIME_AND_PRODUCT_SHAPE_REVIEW.md`

---

## 当前主线
1. 继续把自动注册机从骨架推进成可持续跑的 worker 系统
2. 把邮箱池做成高质量、可扩张、自动治理的资源层
3. 把 automation target discovery / try / mailbox / register 串成闭环
4. 用轻 dashboard + scheduler 提升可观测性与可运维性
5. 后续再接真实指纹浏览器 API，并继续稳定 Codex quota pool 与对外 OpenAI 兼容 API

---

## 当前阻塞
1. 真实指纹浏览器 API 还没接，只做了未来入口占位和内部状态机
2. 邮箱池虽然已能导入/轮询/分层，但压力均衡器、健康看板、自动恢复还没落下
3. automation target 的 try 仍是启发式骨架，不是真实页面自动化
4. OpenClaw 主机安全审计显示 **2 critical / 9 warn**，运行环境仍需整改
5. 主机缺少显式防火墙层，且 `sshd:22` / `cupsd:631` 对外监听

---

## 运行环境审查
### 当前主机条件
- OS：Ubuntu 24.04.4 LTS
- 内核：6.8.0-101-generic
- Rust：1.94.1
- Cargo：1.94.1
- Node：22.22.2
- npm：10.9.7
- Git：2.43.0
- 磁盘：`/` 59G，总已用 39G，剩余 19G
- 内存：3.6GiB，总可用约 1.7GiB
- OpenClaw：2026.4.1，存在 2026.4.2 更新

### 是否符合运行条件
- **项目代码层面：基本符合**
  - 能编译
  - 测试全绿
  - SQLite 本地运行正常
- **主机环境层面：部分符合，但不够稳妥**
  - 资源可跑，但内存余量偏紧
  - 网络与服务可用
  - 安全面不达标，必须整改

### 需要整改
1. 审查或下线被 `openclaw security audit --deep` 标红的插件：
   - `adp-openclaw`
   - `openclaw-weixin`
2. 安装并启用主机防火墙（当前 `ufw` / `nft` 都缺失）
3. 审查 `cupsd`，若不需要打印服务则关闭 631 对外监听
4. 收紧 OpenClaw：
   - `autoAllowSkills`
   - `strictInlineEval`
   - `trustedProxies`
5. 升级 OpenClaw 到 2026.4.2

## 运行形态判断
### 该砍
- GUI 桌面端
- 第三方 provider 主线
- 重前端运营后台

### 该加
- Worker / Scheduler
- 邮箱池健康看板
- 自动恢复 / 解冻策略
- 审计查询接口
