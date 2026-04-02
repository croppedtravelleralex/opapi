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

---

## 下一阶段对象

### Account
用于表示一个上游账号。

### Provider
用于表示一个上游供应商或通道。

### RoutePolicy
用于表示模型到上游的路由规则。

### HealthSnapshot
用于表示上游健康检查快照。

---

## 当前原则

1. 先定义接口需要的对象
2. 再定义调度需要的对象
3. 再定义账号池和健康系统对象
4. 不把长期架构一次性压进 V1
