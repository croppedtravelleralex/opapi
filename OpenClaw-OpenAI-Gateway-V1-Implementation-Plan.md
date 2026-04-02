# OpenClaw OpenAI Gateway Control Plane — V1 实施拆解

## 目标
V1 只解决一件事：**先把平台做成一个可被外部客户端通过 OpenAI 兼容接口调用的最小接入层**，同时保证 **不直接暴露 OpenClaw 内部 WS 入口**。

## V1 范围
### MUST
- `GET /healthz`
- `GET /readyz`
- `GET /v1/models`
- `POST /v1/chat/completions`
- `POST /v1/responses`
- Bearer API Key 鉴权
- 基础 WS bridge
- 基础错误归一化
- request id / 基础日志

### SHOULD
- SSE 流式
- 基础 usage 记录
- Docker Compose / systemd 启动方式
- 最小模型目录配置

### 暂不做
- Account / AccountPool
- WebSession / BrowserProfile
- EgressProfile 动态调度
- 完整 Tenant / Project / RBAC
- 治理平面与 ChangePlan
- 自动化治理

---

## V1 工程目标
### 1. 对外目标
- OpenAI SDK 可通过 `base_url + api_key` 接入
- 至少 `/v1/models`、`/v1/chat/completions`、`/v1/responses` 可用

### 2. 对内目标
- OpenClaw WS 继续只监听 `127.0.0.1:18789`
- 不向公网暴露内部 WS
- 通过网关服务把 HTTP 请求安全桥接到 OpenClaw

### 3. 验收目标
- `curl http://127.0.0.1:8080/healthz` 返回 200
- `curl http://127.0.0.1:8080/readyz` 在上游正常时返回 200
- OpenAI SDK 可成功拉到 `/v1/models`
- 基础 chat/responses 非流式可用
- 鉴权失败返回 401
- 上游不可用返回 503

---

## 推荐技术栈
- **语言**：Rust
- **HTTP 框架**：axum
- **异步运行时**：tokio
- **WS 客户端**：tokio-tungstenite
- **序列化**：serde / serde_json
- **日志**：tracing
- **配置**：dotenvy + env
- **部署**：Caddy + systemd 或 Docker Compose

---

## V1 模块拆分
### 1. Ingress Edge
- Caddy / Nginx
- TLS（开发阶段可先本地 HTTP）

### 2. API Facade
- `/healthz`
- `/readyz`
- `/v1/models`
- `/v1/chat/completions`
- `/v1/responses`

### 3. Auth Middleware
- Bearer API Key 校验
- request id 注入

### 4. WS Bridge
- 建立到 OpenClaw WS 的连接
- 提供最小请求发送与响应接收能力
- 维护基础 ready 检查

### 5. State（最小版）
- 配置
- 模型列表
- 上游连接状态

### 6. Logging / Usage（最小版）
- request id
- path
- model
- latency
- status

---

## 推荐目录结构
```text
openclaw-openai-gateway/
  Cargo.toml
  .env.example
  src/
    main.rs
    app.rs
    config.rs
    error.rs
    state.rs
    routes/
      mod.rs
      health.rs
      models.rs
      chat.rs
      responses.rs
    middleware/
      mod.rs
      auth.rs
      request_id.rs
    bridge/
      mod.rs
      client.rs
      mapper.rs
    domain/
      mod.rs
      openai.rs
      upstream.rs
    observability/
      logging.rs
  tests/
    health_test.rs
    auth_test.rs
    models_test.rs
    chat_test.rs
    responses_test.rs
  edge/
    Caddyfile
  docs/
    roadmap.md
```

---

## 分阶段任务单

### 阶段 1：项目骨架
#### 任务
- 初始化 Rust 项目
- 配置加载
- tracing 日志
- axum router
- request id middleware

#### 完成标准
- 服务可启动
- `/healthz` 可返回 200

---

### 阶段 2：鉴权与基础状态
#### 任务
- Bearer API Key 中间件
- AppState
- WS client skeleton
- `/readyz`

#### 完成标准
- 无 token 返回 401
- `readyz` 能根据上游状态返回 200 / 503

---

### 阶段 3：模型接口
#### 任务
- `/v1/models`
- 基础模型目录（先静态配置）
- OpenAI 风格 list JSON

#### 完成标准
- OpenAI SDK 可识别 models

---

### 阶段 4：chat/responses 最小实现
#### 任务
- `POST /v1/chat/completions`
- `POST /v1/responses`
- 输入标准化
- 基础 WS bridge 请求映射
- 响应映射

#### 完成标准
- 非流式 chat/responses 可用
- 上游失败返回标准错误

---

### 阶段 5：SSE（可选 SHOULD）
#### 任务
- `stream=true` 支持
- SSE 输出
- StreamSession 最小记录（可先内存）

#### 完成标准
- 流式首包可返回
- 中断时错误可解释

---

### 阶段 6：部署与验收
#### 任务
- Caddyfile
- systemd / Docker Compose
- 验收 curl 用例
- README

#### 完成标准
- 本机部署可复现
- 基础验收脚本通过

---

## V1 API 契约（最小版）

### GET /healthz
```json
{ "status": "ok" }
```

### GET /readyz
成功：
```json
{ "status": "ready", "upstream": "ok" }
```
失败：
```json
{
  "error": {
    "message": "upstream unavailable",
    "type": "service_unavailable_error",
    "code": "upstream_unavailable"
  }
}
```

### GET /v1/models
```json
{
  "object": "list",
  "data": [
    {
      "id": "openclaw-default",
      "object": "model",
      "created": 1775116000,
      "owned_by": "openclaw"
    }
  ]
}
```

### POST /v1/chat/completions
最小请求：
```json
{
  "model": "openclaw-default",
  "messages": [
    {"role": "user", "content": "hello"}
  ],
  "stream": false
}
```

### POST /v1/responses
最小请求：
```json
{
  "model": "openclaw-default",
  "input": "hello",
  "stream": false
}
```

---

## V1 错误模型
统一使用 OpenAI 风格错误结构：
```json
{
  "error": {
    "message": "invalid api key",
    "type": "authentication_error",
    "code": "invalid_api_key"
  }
}
```

最少覆盖：
- 401 invalid_api_key
- 404 not_found
- 413 request_too_large
- 429 rate_limited
- 500 internal_error
- 503 upstream_unavailable

---

## V1 测试清单
### 基础接口
- [ ] `/healthz` 返回 200
- [ ] `/readyz` 正常时返回 200
- [ ] `/readyz` 异常时返回 503

### 鉴权
- [ ] 无 token 返回 401
- [ ] 错 token 返回 401
- [ ] 对 token 可通过 `/v1/models`

### 模型接口
- [ ] `/v1/models` 返回标准 list JSON
- [ ] OpenAI SDK 可识别 `base_url + api_key`

### chat/responses
- [ ] chat 非流式返回 200
- [ ] responses 非流式返回 200
- [ ] 上游错误返回标准错误

### 部署
- [ ] Caddy / 本地 HTTP 转发正常
- [ ] OpenClaw WS 未暴露公网

---

## V1 风险点
1. **WS 协议映射不稳定**
   - 先做最小映射，避免过早支持过多参数
2. **错误归一化不完整**
   - 先统一结构，再补细分类
3. **SSE 复杂度偏高**
   - 可先把非流式做稳，再补流式
4. **过早引入多租户/池化导致项目膨胀**
   - V1 明确只做兼容入口 MVP

---

## V1 之后直接衔接的 V2 工作
- Account / AccountPool
- RoutePolicy
- RoutingDecision
- EgressProfile
- 健康分 / 冷却 / fallback
- 账号导入与探活

---

## 本文结论
V1 的目标必须非常克制：**先把它做成一个稳定、最小、可被外部 SDK 调用的 OpenAI 兼容接入层。**
只要这块地基打稳，V2 的资源池化与动态调度才不会返工。
