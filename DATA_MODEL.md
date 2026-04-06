# DATA_MODEL.md

## 数据模型设计（V1 草案）

当前 V1 不急着把完整账号池和调度系统一次做完，先把最小需要的数据对象定清楚。

---

## V1 最小对象

### 1. GatewayConfig
描述当前服务运行配置。

核心字段：
- `host`
- `port`
- `api_title`
- `default_models`

### 2. ModelDescriptor
描述一个对外暴露的模型。

核心字段：
- `id`
- `object`
- `owned_by`
- `upstream`
- `endpoint`
- `provider`（后续补）
- `availability`（后续补）

### 3. ChatCompletionRequest
最小聊天请求对象。

核心字段：
- `model`
- `messages`
- `temperature`（可选）
- `max_tokens`（可选）
- `stream`（可选）

### 4. ChatMessage
最小消息对象。

核心字段：
- `role`
- `content`

### 5. ChatCompletionResponse
最小聊天响应对象。

核心字段：
- `id`
- `object`
- `created`
- `model`
- `choices`

### 6. UpstreamDescriptor
描述一个静态上游配置槽位。

核心字段：
- `name`
- `base_url`
- `append_v1`
- `api_key`（仅运行时使用，不对外暴露）

### 7. AccountRecord
描述最小账号池条目（当前已开始代码落地）。

核心字段：
- `id`
- `provider`
- `label`
- `status`
- `base_url`
- `model_scope`

---

## 下一阶段对象

### Account
用于表示一个上游账号。

建议最小字段：
- `id`
- `provider`
- `label`
- `status`
- `credential_ref`
- `quota_state`
- `last_health_at`

### Provider
用于表示一个上游供应商或通道。

建议最小字段：
- `id`
- `name`
- `base_url`
- `kind`
- `supports_models`

### RoutePolicy
用于表示模型到上游的路由规则。

建议最小字段：
- `model`
- `candidate_upstreams`
- `priority`
- `fallback_policy`

### HealthSnapshot
用于表示上游健康检查快照。

建议最小字段：
- `target`
- `status`
- `latency_ms`
- `error_rate`
- `checked_at`

---

## 当前原则

1. 先定义接口需要的对象
2. 再定义调度需要的对象
3. 再定义账号池和健康系统对象
4. 不把长期架构一次性压进 V1
5. 当前阶段优先围绕 **静态多上游可解释性 + 最小错误处理 + 后续账号池扩展接口** 来扩模型
6. 当前账号池代码先只落 **本地 SQLite + 最小 account store**，不提前把完整调度系统压进来
