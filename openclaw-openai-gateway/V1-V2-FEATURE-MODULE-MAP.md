# OpenClaw OpenAI Gateway — V1 / V2 功能清单与代码模块对应表

## 结论
当前最优推进方式不是继续抽象描述，而是把 **V1 / V2 的能力项、交付标准、代码落点、优先级** 一次钉死。

---

# 一、V1 功能清单与代码模块对应

## V1-A：OpenAI 兼容入口

### 1. `/v1/models`
- **目标**：返回统一模型目录，供 OpenAI SDK / 客户端拉取
- **完成标准**：
  - 返回 OpenAI 风格 `object=list`
  - 支持模型别名与展示名
  - 至少支持静态模型目录
- **代码落点**：
  - `src/routes/models.rs`
  - `src/domain/openai.rs`
  - `src/domain/models.rs`
  - `src/state.rs`
- **优先级**：P0

### 2. `/v1/chat/completions`
- **目标**：最小非流式聊天接口可用
- **完成标准**：
  - 兼容 OpenAI chat 请求结构
  - 能桥接到内部 provider executor
  - 返回统一 chat completion 结构
- **代码落点**：
  - `src/routes/chat.rs`
  - `src/domain/openai.rs`
  - `src/bridge/mapper.rs`
  - `src/providers/*`
- **优先级**：P0

### 3. `/v1/responses`
- **目标**：最小 responses 接口可用
- **完成标准**：
  - 支持最小 input -> output 桥接
  - 返回统一 responses 结构
- **代码落点**：
  - `src/routes/responses.rs`
  - `src/domain/openai.rs`
  - `src/bridge/mapper.rs`
  - `src/providers/*`
- **优先级**：P0

### 4. SSE 流式（后补）
- **目标**：支持 `stream=true`
- **完成标准**：
  - 首包能正常返回
  - 中断能解释
  - 基础 SSE 事件格式正确
- **代码落点**：
  - `src/routes/chat.rs`
  - `src/routes/responses.rs`
  - `src/bridge/stream.rs`
- **优先级**：P1

---

## V1-B：鉴权与边界

### 5. Bearer API Key 鉴权
- **目标**：保护业务面 API
- **完成标准**：
  - 无 token 返回 401
  - 错 token 返回 401
  - 正确 token 可通过
- **代码落点**：
  - `src/middleware/auth.rs`
  - `src/config.rs`
- **优先级**：P0

### 6. 管理面与业务面边界预留
- **目标**：后续治理面独立，不和业务面混用
- **完成标准**：
  - 先在目录和路由层预留管理面空间
- **代码落点**：
  - `src/routes/admin/`
  - `src/governance/`
- **优先级**：P2

---

## V1-C：Provider 抽象与桥接

### 7. Provider 抽象最小版
- **目标**：不要把上游逻辑写死在 handler 里
- **完成标准**：
  - 至少抽出统一 provider trait / adapter
  - 支持 API provider / Gateway provider / Local provider 最小骨架
- **代码落点**：
  - `src/providers/mod.rs`
  - `src/providers/api.rs`
  - `src/providers/gateway.rs`
  - `src/providers/local.rs`
- **优先级**：P0

### 8. OpenClaw / 上游 Bridge
- **目标**：把 HTTP 请求映射到内部执行器
- **完成标准**：
  - readiness 探测真实可用
  - chat / responses 可最小桥接
- **代码落点**：
  - `src/bridge/client.rs`
  - `src/bridge/mapper.rs`
  - `src/bridge/readiness.rs`
- **优先级**：P0

---

## V1-D：健康、错误、日志

### 9. `healthz`
- **目标**：进程活着
- **完成标准**：固定返回 200 + status ok
- **代码落点**：
  - `src/routes/health.rs`
- **优先级**：P0

### 10. `readyz`
- **目标**：上游真的可用
- **完成标准**：
  - 不再依赖 fake check
  - 能基于真实 provider / bridge 状态返回 200/503
- **代码落点**：
  - `src/routes/health.rs`
  - `src/bridge/readiness.rs`
  - `src/state.rs`
- **优先级**：P0

### 11. 统一错误模型
- **目标**：上游失败不把内部脏错误直接暴露给客户端
- **完成标准**：
  - 401 / 429 / 5xx / upstream unavailable 统一收口
  - 错误 type / code 稳定
- **代码落点**：
  - `src/error.rs`
  - `src/bridge/mapper.rs`
- **优先级**：P0

### 12. request id / trace id / 基础日志
- **目标**：先具备最小可排障能力
- **完成标准**：
  - 每个请求有 request id
  - 日志里能看到 path / model / provider / latency / error type
- **代码落点**：
  - `src/middleware/request_id.rs`
  - `src/observability/logging.rs`
- **优先级**：P0

---

## V1-E：部署与测试

### 13. Docker Compose / systemd
- **目标**：最小可复现部署
- **完成标准**：
  - `.env.example`
  - `docker-compose.yml`
  - systemd service 示例
- **代码落点**：
  - `edge/`
  - `docs/`
  - 根目录部署文件
- **优先级**：P1

### 14. 最小测试
- **目标**：避免每次改动都靠手试
- **完成标准**：
  - auth 测试
  - health 测试
  - models 测试
  - chat/responses 最小测试
- **代码落点**：
  - `tests/auth_test.rs`
  - `tests/health_test.rs`
  - `tests/models_test.rs`
  - `tests/chat_test.rs`
  - `tests/responses_test.rs`
- **优先级**：P1

---

# 二、V2 功能清单与代码模块对应

## V2-A：资源池

### 1. Account 导入
- **目标**：账号不再散落在配置里
- **完成标准**：
  - 单账号导入
  - 批量导入
  - 导入预校验
  - 导入后探活
- **代码落点**：
  - `src/domain/account.rs`
  - `src/repositories/account_repo.rs`
  - `src/automation/account_probe.rs`
- **优先级**：P0

### 2. AccountPool
- **目标**：账号进入池化管理
- **完成标准**：
  - 分组
  - 标签
  - 生命周期状态
  - 冷却 / 恢复 / 隔离
- **代码落点**：
  - `src/domain/account_pool.rs`
  - `src/repositories/account_pool_repo.rs`
  - `src/automation/pool_governor.rs`
- **优先级**：P0

### 3. ProviderPool
- **目标**：Provider 统一池化管理
- **完成标准**：
  - provider class
  - 健康状态
  - 风险状态
  - 权重
- **代码落点**：
  - `src/domain/provider.rs`
  - `src/domain/provider_pool.rs`
  - `src/repositories/provider_repo.rs`
- **优先级**：P0

### 4. EgressProfile
- **目标**：出口不再隐形存在
- **完成标准**：
  - egress profile
  - 地区标记
  - 健康状态
  - 风险状态
- **代码落点**：
  - `src/domain/egress.rs`
  - `src/repositories/egress_repo.rs`
- **优先级**：P1

---

## V2-B：模型目录与能力画像

### 5. ModelCatalog
- **目标**：模型成为一等公民对象
- **完成标准**：
  - canonical model
  - alias
  - provider binding
  - availability
- **代码落点**：
  - `src/domain/model.rs`
  - `src/repositories/model_repo.rs`
- **优先级**：P0

### 6. 模型能力画像
- **目标**：为路由做基础数据支撑
- **完成标准**：
  - 能力标签
  - 成本画像
  - 延迟画像
  - 稳定性画像
- **代码落点**：
  - `src/domain/model_profile.rs`
  - `src/ops/model_metrics.rs`
- **优先级**：P1

---

## V2-C：动态路由

### 7. RoutingPolicy
- **目标**：路由规则不再散落在 handler 或配置字符串里
- **完成标准**：
  - 模型到 provider class 规则
  - 权重 / 优先级
  - fallback 规则
- **代码落点**：
  - `src/routing/policy.rs`
  - `src/routing/selector.rs`
- **优先级**：P0

### 8. RoutingDecision
- **目标**：每次路由可记录、可解释
- **完成标准**：
  - 记录选中的 provider / pool / egress
  - 记录未选原因
  - 记录 fallback 原因
- **代码落点**：
  - `src/routing/decision.rs`
  - `src/observability/explain.rs`
- **优先级**：P0

### 9. Fallback Chain
- **目标**：主链失败时有明确备链
- **完成标准**：
  - 主备候选列表
  - fallback 顺序
  - 熔断 / 降级钩子
- **代码落点**：
  - `src/routing/fallback.rs`
- **优先级**：P0

---

## V2-D：Explainability 与最小治理

### 10. Explainability v1
- **目标**：系统不做黑盒路由
- **完成标准**：
  - 能解释为什么选这个 provider
  - 能解释为什么没选另一个
  - 能解释为什么 fallback / reject
- **代码落点**：
  - `src/observability/explain.rs`
  - `src/routing/decision.rs`
- **优先级**：P0

### 11. Audit Event v1
- **目标**：关键动作可回放
- **完成标准**：
  - 导入账号
  - 修改路由策略
  - 触发 fallback
  - provider 冷却
- **代码落点**：
  - `src/governance/audit.rs`
- **优先级**：P1

---

# 三、当前最优实现顺序

## 第一组：现在立刻做
1. fake `readyz` → 真实上游探测
2. fake `chat` → 最小真实 bridge
3. fake `responses` → 最小真实 bridge
4. Provider 抽象最小版

## 第二组：紧接着做
1. ModelCatalog 最小骨架
2. ProviderPool 最小骨架
3. RoutingPolicy / RoutingDecision 最小骨架
4. Explainability v1

## 第三组：再往后做
1. Account / AccountPool
2. EgressProfile
3. Audit Event v1
4. Docker / systemd / tests

---

# 四、当前结论

当前最值动作不是继续扩文档，而是：

**把 V1 的真实 bridge 做出来，同时把 V2 的路由与资源池骨架立住。**

只要这两步完成，项目就会真正从“设计期”进入“平台骨架落地期”。
