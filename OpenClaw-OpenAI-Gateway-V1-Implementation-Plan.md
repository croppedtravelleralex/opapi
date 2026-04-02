# OpenClaw OpenAI Gateway Control Plane — V1 到 V4 实施计划

## 1. 实施原则

### 1.1 总原则
实现顺序必须服从一个核心原则：

**先立平台主梁，再补平台能力；先做最小闭环，再做自动化与治理增强。**

### 1.2 六根主梁
后续所有工程动作都围绕这六根主梁展开：
1. Provider 抽象
2. Account / Session / Pool 抽象
3. Model + Routing 抽象
4. Egress / IP 抽象
5. Security / Governance 抽象
6. Automation / Ops 抽象

### 1.3 目标
不是做一个“能跑的 HTTP 代理”，而是逐步做成一个：
- 对外兼容 OpenAI
- 对内统一调度多类上游
- 安全、可治理、可观测、可运营
- 能逐步演进为自动化平台

---

## 2. MUST / SHOULD / COULD 总表

### MUST
- OpenAI 兼容入口：`/v1/models`、`/v1/chat/completions`、`/v1/responses`
- Bearer API Key 鉴权
- 基础 Provider 抽象
- 静态模型目录与最小模型路由
- 基础错误归一化
- request id / trace id
- Health / Readiness
- 基础日志与最小验收流程

### SHOULD
- SSE 流式
- 基础 usage 记录
- Docker Compose / systemd
- 最小 Egress 抽象预留
- 配置版本化最小骨架
- Mock Upstream / Mock Provider / Replay

### COULD
- 账号导入与 Warmup Pool
- Browser Session Pool
- Explainability UI
- 成本统计
- 容量预测
- 自动日报 / 自动诊断
- 多协议兼容扩展

---

## 3. V1：最小可用接入层

## 3.1 目标
V1 只解决一件事：

**先把平台做成一个可被外部客户端通过 OpenAI 兼容接口调用的最小接入层。**

## 3.2 V1 范围
### MUST
- `GET /healthz`
- `GET /readyz`
- `GET /v1/models`
- `POST /v1/chat/completions`
- `POST /v1/responses`
- Bearer API Key 鉴权
- Provider 抽象最小版
- 基础错误归一化
- request id / trace id / 基础日志

### SHOULD
- SSE 流式
- 基础 usage 记录
- Docker Compose / systemd
- Mock Upstream

### 暂不做
- Account / AccountPool
- Browser Session Pool
- 动态 Egress 调度
- Tenant / Project / RBAC 全量
- 配置治理全套能力
- 自动化治理

## 3.3 V1 主要模块
1. Ingress Edge
2. API Facade
3. Auth Middleware
4. Provider Client / Bridge
5. App State
6. Logging / Usage

## 3.4 V1 验收标准
- OpenAI SDK 可通过 `base_url + api_key` 接入
- `/v1/models`、`/v1/chat/completions`、`/v1/responses` 可用
- 鉴权失败返回 401
- 上游不可用返回标准错误
- 本机部署可复现

---

## 4. V2：资源池与动态调度起步

## 4.1 目标
把系统从“接口兼容层”推进到“受控资源调度层”。

## 4.2 V2 范围
### MUST
- Account 导入
- AccountPool
- ModelCatalog
- ProviderPool
- Routing Policy 最小版
- Fallback 链最小版
- EgressProfile 抽象预留

### SHOULD
- 导入预校验
- 导入后自动探活
- 账号分组与标签
- 冷却 / 恢复 / 隔离状态
- 路由解释最小版

### COULD
- Warmup Pool / Quarantine Pool
- 成本画像 / 延迟画像
- 模型可用性自动识别

## 4.3 V2 验收标准
- 支持多个 Provider 池化管理
- 模型不再完全写死映射
- 能根据最小规则选择候选 Provider
- 能记录一条请求的路由决策依据

---

## 5. V3：Web Provider 与治理增强

## 5.1 目标
把“高风险但高价值”的 Web Provider 纳入统一平台边界。

## 5.2 V3 范围
### MUST
- BrowserProfile
- WebSession
- Browser Session Pool
- 登录态检测
- Challenge / 异常页检测
- 自动冷却 / 自动降权
- Explainability 增强
- Config Snapshot / Rollback 最小版

### SHOULD
- Session Warmup
- 自动 Profile 重建
- 路由解释存档
- ChangePlan / ReleaseRecord
- Shadow Routing / Canary 预留

### COULD
- 截图留证
- 失败摘要自动生成
- DOM 结构漂移分类

## 5.3 V3 验收标准
- Web Provider 可被统一纳管
- Web Session 能被健康检查与冷却
- 变更可记录、可回滚
- 路由决策可解释

---

## 6. V4：自动化与运营平台化

## 6.1 目标
把平台从“可控”推进到“可长期运营”。

## 6.2 V4 范围
### MUST
- 自动探活
- 自动健康治理
- 自动冷却 / 恢复 / 隔离 / 迁池
- 成本统计
- 容量看板
- Provider SLA / 池级 SLO
- 故障时间线
- 风险日报 / 故障复盘草稿

### SHOULD
- 自动策略优化建议
- 成本替代建议
- 容量预测
- 热模型容量储备建议

### COULD
- 智能诊断
- 自动日报
- 自适应权重
- 智能运营助手

## 6.3 V4 验收标准
- 健康治理不再依赖人工盯盘
- 平台具备基本成本/容量/稳定性运营能力
- 风险与故障具备可追溯时间线
- 自动化建议可辅助人工决策

---

## 7. 按四条主线拆阶段

## 7.1 自动化主线
### Phase A1：基础自动化
- 导入后自动探活
- 账号自动打标签
- 模型自动发现
- Provider 自动健康检测

### Phase A2：资源自动治理
- 自动冷却 / 恢复
- 自动迁池
- Session Warmup
- 出口异常自动摘除

### Phase A3：策略自动化
- 自动权重调整
- 自动 Fallback 建议
- 自动低成本建议
- 自动异常分类

### Phase A4：智能运营
- 自动日报
- 自动复盘草稿
- 自动容量预警
- 自动风险总结

## 7.2 安全主线
### Phase S1：基础边界
- 业务面鉴权
- 管理面隔离
- OpenClaw 仅本机监听
- 80/443 收口

### Phase S2：凭证与执行隔离
- Secret Ref
- Provider 隔离
- Browser Profile 隔离
- Worker 隔离

### Phase S3：IP / 出口安全
- 入口/出口分层
- Egress Profile
- 风险出口隔离
- 地区匹配策略

### Phase S4：行为风控
- 异常请求检测
- 高失败租户限流
- Challenge 检测
- 高风险 Provider 自动降权

## 7.3 治理主线
### Phase G1：配置治理
- 配置模型化
- 配置版本化
- 配置快照
- 配置回滚

### Phase G2：权限治理
- Tenant / Project
- RBAC
- 审批流
- 敏感操作二次确认

### Phase G3：策略治理
- 模型策略版本
- 池策略版本
- 发布记录
- 灰度 / Canary

### Phase G4：审计治理
- 操作留痕
- 审计导出
- 路由解释存档
- 变更复盘

## 7.4 运营主线
### Phase O1：基础观测
- 请求日志
- 成功率
- 延迟
- 错误分类

### Phase O2：资源运营
- 池容量看板
- Provider 健康排行
- 出口健康排行
- Session 容量监控

### Phase O3：成本运营
- 每模型成本
- 每租户成本
- 每 Provider 成本
- 成本异常预警

### Phase O4：稳定性运营
- SLA / SLO
- 错误预算
- 故障时间线
- 恢复时长统计

---

## 8. 模块边界与调用关系

## 8.1 顶层模块
1. Ingress Layer
2. OpenAI API Layer
3. Routing Layer
4. Execution Layer
5. State Layer
6. Governance Layer
7. Ops Layer

## 8.2 顶层调用流
`Client -> OpenAI Compatible API -> Auth/Tenant -> Routing Engine -> Provider Pool -> Executor -> Upstream`

并行写入：
- Health State
- Audit
- Metrics
- Config Snapshot

## 8.3 工程主干目录建议
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
    middleware/
    bridge/
    domain/
    routing/
    providers/
    governance/
    ops/
    automation/
  tests/
  docs/
  edge/
```

---

## 9. 风险与控制措施

### 技术风险
1. 上游协议变动 → Adapter / Capability 探测 / 回归测试
2. 流式链路脆弱 → 流式状态机 / 首尾包监控 / Fail Closed
3. 动态路由过度复杂 → 路由分层 / Explainability / 限规则集

### 安全风险
4. 凭证泄漏 → Secret Ref / 默认脱敏 / 最小权限 / 禁明文导出
5. 管理面暴露 → 内网/Tailscale / 分权 / 二次确认 / 审计
6. Web Provider 风险高 → Profile 隔离 / Challenge 检测 / 自动冷却
7. IP 风险污染 → 入口出口分离 / 高风险池独立 / 健康评分

### 运营风险
8. 资源池耗尽 → 容量看板 / 阈值预警 / 自动 Warmup / Fallback Pool
9. 成本失控 → Tenant 配额 / 模型预算 / 成本看板 / 替代建议
10. 故障不可解释 → Request/Trace ID / 决策记录 / 时间线

### 产品风险
11. 目标过大不落地 → 分层路线图 / MVP 最小闭环 / 蓝图与交付分开
12. 功能堆积失控 → MUST/SHOULD/COULD / 每阶段只收口主干

---

## 10. 当前最优推进顺序

### P0
1. 把 PRD v3 的平台定位、能力矩阵、数据模型、主线拆解彻底收口
2. 把当前工程骨架目录对齐到六根主梁
3. 跑通 V1 最小闭环：health / ready / models / chat / responses
4. 收口 Provider 抽象与错误模型

### P1
1. 建立 ModelCatalog / ProviderPool / RoutingPolicy 最小骨架
2. 建立 Account / AccountPool 最小骨架
3. 建立 Explainability 与 Audit 最小记录
4. 补 Docker / systemd / Mock / Replay

### P2
1. 建立 Web Provider / Browser Session Pool
2. 建立 Config Snapshot / Rollback / ChangePlan
3. 建立自动探活与自动冷却主干
4. 建立基础运营与成本看板

---

## 11. 当前结论

当前最值动作不是继续散补功能，而是：

**先把平台蓝图、主梁抽象、V1–V4 路线图和 MUST/SHOULD/COULD 一次钉死。**

这样后面代码推进就不会在“代理 / 网关 / 控制面 / 平台”之间反复摇摆。 
