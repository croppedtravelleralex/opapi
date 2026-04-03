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
3. 额度池挑选与最小路由
4. 对外 OpenAI 兼容 API

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
cp .env.example .env
cargo run
```

---

## 当前不该扩张的方向
- 第三方 provider 接入市场
- 大型治理后台
- release/config/change-plan 复杂控制面
- 与本机长期运行无关的花活模块
