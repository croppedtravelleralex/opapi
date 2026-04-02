# FINAL_FEATURE_MAP.md

## 结论
`openclaw-openai-gateway` 的最终目标不是一个简单的 OpenAI 兼容转发器，而是一个 **多上游 AI 能力接入、路由调度、资源池治理、自动化运营** 的统一平台。

当前应使用 **两套进度口径**：
- **骨架工程进度**：衡量主干结构、模块边界、最小验证闭环是否成立
- **最终功能进度**：衡量最终目标能力到底实现了多少

当前判断：
- **骨架工程进度：84%–90%**
- **最终功能进度：25%–35%**

---

## 一、统一 OpenAI 兼容接入层

### 最终要实现
- `GET /v1/models`
- `POST /v1/chat/completions`
- `POST /v1/responses`
- 后续 `embeddings / images / audio`
- SSE 流式
- OpenAI 风格错误格式
- 兼容 OpenAI SDK / 各类 BaseURL 客户端

### 当前状态
- `models / chat / responses` 已有最小可用版
- 非流式最小闭环已具备
- SSE / embeddings / images / audio 仍未开始

### 当前真实进度
- **骨架进度：78%**
- **最终功能进度：35%**

---

## 二、多 Provider 接入层

### 最终要实现
- `API Provider`
- `Gateway Provider`
- `Web Session Provider`
- `Local Provider`
- 统一 provider adapter / bridge / mapper
- 不同认证方式、返回结构、协议统一收口

### 当前状态
- `gateway / api / local` 最小 provider skeleton 已有
- 当前真正跑通的主要还是 `Gateway Provider`
- 真实多 provider 行为还没有形成

### 当前真实进度
- **骨架进度：72%**
- **最终功能进度：20%**

---

## 三、模型目录与能力画像

### 最终要实现
- `ModelCatalog`
- canonical model / alias
- provider capability
- model availability
- context size / stream / tools / embeddings 能力画像
- 稳定性 / 延迟 / 成本画像

### 当前状态
- `ModelCatalog` 最小对象已落下
- SQLite 中 `model_catalog` 已可落表并 seed
- `provider_capabilities / model_availability` 还只是目标，未形成真实读写与路由使用

### 当前真实进度
- **骨架进度：76%**
- **最终功能进度：28%**

---

## 四、路由与调度控制平面

### 最终要实现
- `RoutingPolicy / RoutingDecision`
- provider 选择
- model-provider 绑定
- fallback / failover / 熔断 / 冷却 / 降级
- 成本 / 延迟 / 风险 / 可用性联合决策
- explainability / auditability

### 当前状态
- `RoutingPolicy / RoutingDecision` 已有最小版
- `x-routing-explain` 已落下
- 当前仍接近“单 provider + 默认选择”
- 动态调度、熔断、降级还未真正实现

### 当前真实进度
- **骨架进度：80%**
- **最终功能进度：22%**

---

## 五、账号池 / ProviderPool / 会话池

### 最终要实现
- `AccountPool`
- `ProviderPool`
- `BrowserSessionPool / WebSessionPool`
- 导入、标签、生命周期、冷却、恢复、隔离
- 池化调度

### 当前状态
- `ProviderPool` 最小对象已落下
- `AccountPool / SessionPool` 基本还未开始

### 当前真实进度
- **骨架进度：38%**
- **最终功能进度：10%**

---

## 六、治理与运维层

### 最终要实现
- audit log
- explainability
- config snapshot
- release record
- change plan
- 健康检查、ready、诊断接口
- 指标、告警、灰度、回滚

### 当前状态
- `healthz / readyz` 已有
- explainability v1 已有
- audit skeleton 已有，但主要仍在内存层
- config snapshot / release / change plan 未开始

### 当前真实进度
- **骨架进度：68%**
- **最终功能进度：18%**

---

## 七、持久化与数据层

### 最终要实现
- SQLite / 后续可扩 Postgres
- model / provider / capability / availability / account / audit 等表
- migration 管理
- repository 真读写
- 数据一致性与回填

### 当前状态
- SQLite 已真实接入
- `model_catalog / providers` 已能建表并 seed
- `/v1/models` 已切到 SQLite-backed reads
- 其余 repository 仍以内存为主，SQLite 仍未成为全面主读写底座

### 当前真实进度
- **骨架进度：82%**
- **最终功能进度：32%**

---

## 八、自动化运营层

### 最终要实现
- 自动探活
- 自动能力识别
- 自动可用性更新
- 自动风险识别
- 自动调度策略调整
- 后台任务 / cron / maintenance job

### 当前状态
- 基本未开始

### 当前真实进度
- **骨架进度：12%**
- **最终功能进度：5%**

---

## 总体修正结论

### 旧说法（不准确）
- “最终总进度 84%”

### 新口径（准确）
- **骨架工程进度：84%–90%**
- **最终功能进度：25%–35%**

原因：
当前完成的主要是 **主梁、模块、最小读写、最小测试、最小 bridge、最小 SQLite**，而不是完整平台能力。

---

## 当前最缺的 4 大块
1. **真实多 Provider 接入与真实上游协议对接**
2. **真正可工作的动态路由/降级/熔断/调度**
3. **AccountPool / SessionPool / ProviderPool 完整化**
4. **capability / availability / audit / governance 真正落库并参与决策**

---

## 当前最合适的推进顺序
1. **先把 `providers` 主读源切到 SQLite，并补 `provider_capabilities / model_availability` 真落表与 seed**
2. **再把 routing 决策真正接到 SQLite 数据，而不是只用默认值**
3. **再补 AccountPool / Governance / Audit 持久化**
4. **最后再扩自动化运营层与更多 OpenAI 能力面**
