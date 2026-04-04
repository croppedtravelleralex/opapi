# openclaw-openai-gateway

## 结论
这个项目现在应当收成：

**最适合本机长期运行的单机轻服务：SQLite + API + Codex 额度主链 + 最小调度。**

不再按“大平台 / 重控制面 / 花哨 provider 市场”去做。

---

## 最适合本机的项目形态
### 保留
- Rust API Server
- SQLite 单机存储
- `Codex App / Codex Web` 额度采集主链
- `quota_snapshots -> admission -> pool_members -> chat/responses`
- 轻量 session bridge
- 最小 auth / request-id / audit
- 后续可补一个轻 scheduler

### 暂不做
- GUI 桌面端
- 重 dashboard
- governance 控制面接口
- 第三方 provider 扩展
- 大而全平台化模块

---

## 当前主线
1. `Codex App` 额度反代
2. `Codex Web` 额度反代
3. 自动注册机（母号 / 子号 / 邀请 / 验证 / 入池）
4. 邮箱池（导入 / 轮询 / 质量治理 / 自动分层）
5. 自动化目标发现与尝试
6. 对外 OpenAI 兼容 API

---

## 当前服务边界
### 对外接口
- `GET /healthz`
- `GET /readyz`
- `GET /v1/models`
- `GET /v1/providers`
- `GET /v1/codex/quota-sources`
- `GET /v1/codex/quota-overview`
- `POST /v1/codex/quota/collect`
- `POST /v1/codex/automation-targets/discover`
- `POST /v1/codex/automation-targets/try`
- `POST /v1/codex/auto-register`
- `POST /v1/codex/auto-register/dispatch`
- `POST /v1/codex/auto-register/worker/run`
- `POST /v1/codex/auto-register/autoloop/run`
- `POST /v1/codex/auto-register/dead-letter/recover`
- `POST /v1/mailboxes/import`
- `GET /v1/mailboxes/overview`
- `POST /v1/mailboxes/expand`
- `POST /v1/mailboxes/tiering/run`
- `POST /v1/mailboxes/poll/run`
- `POST /v1/chat/completions`
- `POST /v1/responses`

### 内部保留能力
- quota parser / collector
- admission
- pool repo / pool router
- source context
- session bridge
- executor
- sqlite repositories
- registration task queue / safety state machine
- verification task queue
- managed mailbox pool / mailbox bindings / poll runs
- mailbox capacity events / 自动升降级规则
- automation target discovery / try / attempt history

---

## 运行建议
### 本机长期运行的最优形态
- 单进程 API 服务
- SQLite 本地文件
- 无 GUI
- 无重前端
- 无多余控制面
- 后续只补轻量 cron / scheduler

### 当前启动
```bash
cargo run
```

### 当前关键运行条件
- OS：Ubuntu 24.04 LTS 或同级 Linux
- Rust：`rustc 1.94+`
- Node：`v22+`
- SQLite：本地文件可写
- 内存：建议 >= 4GB（当前主机 3.6GiB，可跑但余量不大）
- 磁盘：建议保留 >= 10GB 可用空间（当前 `/` 约剩 19GB）
- 网络：
  - 本地 OpenClaw gateway 可达
  - 如要走真实指纹浏览器，后续需配置浏览器入口 API
  - dashboard 只建议绑定 `127.0.0.1`，通过 **SSH 端口转发**远程访问，不直接公网开放
- 安全前提：
  - API 走 Bearer 鉴权
  - 邮箱导入接口不回显明文凭据
  - 生产环境不建议把控制面直接暴露到公网

### 当前已知运行风险 / 整改项
- OpenClaw `security audit --deep` 有 **2 critical / 9 warn**
- 当前主机发现：
  - `sshd` 对外监听 `0.0.0.0:22`
  - `cupsd` 对外监听 `0.0.0.0:631`
  - OpenClaw 网关本身仍是 loopback，但通过 Tailscale Serve 暴露
  - `ufw` / `nft` 未安装，说明主机缺少显式防火墙层
- 当前项目代码自身可运行，`cargo test -q` 通过（17 + 17 + 32），但主机安全面仍需整改

### 建议整改顺序
1. 先处理 OpenClaw 高危插件审查 / 下线
2. 给主机补上防火墙（至少限制 22 / 631）
3. 明确是否真的需要 CUPS，对外不需要就关掉 631
4. 收紧 OpenClaw `autoAllowSkills` / `strictInlineEval` / `trustedProxies`
5. 升级 OpenClaw 到 `2026.4.2`

### 新增轻运维面板 / 调度入口
- `GET /ops/overview`
- `GET /ops/dashboard`
- `POST /ops/scheduler/tick`

说明：
- `dashboard` 目标是**轻运维观察面**，不是大后台
- 推荐访问方式：
```bash
ssh -L 8088:127.0.0.1:<服务端口> <你的服务器>
```
- 然后本机浏览器打开：
  - `http://127.0.0.1:8088/ops/dashboard`

---

## 当前不该扩张的方向
- 第三方 provider 接入市场
- 大型治理后台
- release/config/change-plan 复杂控制面
- 与本机长期运行无关的花活模块
