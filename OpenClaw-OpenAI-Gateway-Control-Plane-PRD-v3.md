# OpenClaw OpenAI Gateway Control Plane PRD v3

## 1. 产品重新定义

### 1.1 一句话定义
OpenClaw OpenAI Gateway Control Plane 不是“兼容 OpenAI 的入口”或“桥接 OpenClaw 的网关”的增强版，而是一个**多上游 AI 能力接入与自动化运营平台**。

### 1.2 产品本质
它同时覆盖以下八层能力：
1. **接入层**
2. **调度层**
3. **自动化层**
4. **安全层**
5. **治理层**
6. **观测层**
7. **运营层**
8. **演进层**

### 1.3 更高阶形态
从长期形态看，它应收口为：

**AI Gateway + Routing Brain + Provider Control Plane + Security Layer + Ops Platform**

更直白地说：

**不是转发器，而是 AI 能力调度中枢。**

### 1.4 核心目标
平台最终要同时解决六件事：
- 对外提供统一 OpenAI 兼容入口
- 对内纳管 API / Gateway / Web / Local 多类上游
- 统一调度账号、模型、Provider、Session、出口与策略
- 把安全边界前置，而不是后补
- 把治理、审计、灰度、回滚做成控制平面能力
- 把自动化与运营做成长期主干，而不是人工补锅

---

## 2. 核心价值与边界

### 2.1 核心价值
平台价值不在于“转发成功一次请求”，而在于：
- **可接入**：统一兼容外部客户端
- **可调度**：不同模型/Provider/出口按策略分配
- **可治理**：配置、策略、权限、变更可版本化与回滚
- **可运营**：健康、成本、容量、稳定性可观测可优化
- **可自动化**：账号、模型、出口、Session、策略尽量自动维护
- **可演进**：后续加新 Provider / 新执行器 / 新协议不推倒重来

### 2.2 非目标
当前不以以下内容为优先：
- 一开始就做完整 UI 控制台
- 一开始就做跨地域多活
- 一开始就做复杂 DSL 规则引擎
- 一开始就覆盖所有协议与所有多模态接口
- 一开始就实现完全自治系统

### 2.3 产品边界
本平台：
- **负责接入、调度、治理、运营**
- **不替代上游模型本身**
- **不以绕过上游安全边界为目标**
- **不默认承诺所有上游永远稳定**

---

## 3. 平台总能力矩阵

平台最终能力分为四大层：**核心接入、资源与调度、安全与治理、自动化与运营**。

### A. 核心接入层

#### 3.1 OpenAI 兼容 API
- `GET /v1/models`
- `POST /v1/chat/completions`
- `POST /v1/responses`
- `POST /v1/embeddings`
- SSE 流式输出
- 统一错误结构与状态码映射

#### 3.2 多 Provider 接入
- API Provider
- Gateway Provider
- Web Session Provider
- Local Provider

#### 3.3 协议桥接
- HTTP → 内部统一 DTO
- DTO → API / WS / Browser / Local Executor
- 统一请求映射
- 统一响应映射
- 统一错误映射

#### 3.4 兼容性层
- OpenAI SDK 兼容
- 客户端差异兼容
- 协议版本演进兼容

### B. 资源与调度层

#### 3.5 账号导入
- 单账号导入
- 批量导入
- 导入预校验
- 导入后探活
- 导入失败标红

#### 3.6 账号池
- 分组
- 标签
- 生命周期状态
- 冷却 / 恢复 / 隔离
- Warmup Pool / Production Pool / Quarantine Pool

#### 3.7 模型目录
- 模型注册
- 模型别名
- 能力画像
- 成本画像
- 延迟画像
- 稳定性画像
- 可用 Provider 关系

#### 3.8 Provider Pool
- API / Gateway / Web / Local 统一池化
- Provider Class 分类
- 健康状态
- 风险状态
- 权重与优先级

#### 3.9 Browser Session Pool
- Browser Profile 池
- Web Session 池
- Session Warmup
- 健康检查
- Profile 清理 / 重建

#### 3.10 动态路由引擎
- 模型动态分配
- Provider Class 选择
- 主备候选生成
- 熔断 / 降级 / Fallback
- Explainable Routing

#### 3.11 出口与 IP 池
- Egress Profile
- 地区匹配
- 风险隔离
- 健康打分
- 自动摘除异常出口

### C. 安全与治理层

#### 3.12 鉴权与权限
- API Key
- Tenant / Project
- RBAC
- 模型白名单
- 路径级权限
- 方法级权限

#### 3.13 限流与配额
- Key 限流
- Tenant 限流
- 模型限流
- 并发限制
- 请求体限制

#### 3.14 凭证安全
- Secret Ref
- 凭证轮换
- 过期检测
- 最小权限访问
- 默认脱敏展示
- 普通日志禁明文

#### 3.15 Web Provider 风控
- 登录失效检测
- Challenge / CAPTCHA 检测
- DOM 结构变化检测
- 自动降权 / 冷却 / 隔离

#### 3.16 配置治理
- 配置版本化
- 配置 Diff
- 灰度发布
- 回滚快照
- 不安全默认配置检测
- 风险项扫描

#### 3.17 审计与解释
- Request ID / Trace ID
- 路由解释
- 操作留痕
- 审计导出
- 变更复盘

### D. 自动化与运营层

#### 3.18 自动探活
- 账号探活
- Provider 探活
- 出口探活
- Session 探活

#### 3.19 自动健康治理
- 自动冷却
- 自动恢复
- 自动隔离
- 自动迁池
- 自动补齐目标池容量

#### 3.20 自动策略优化
- 池权重调整建议
- Fallback 顺序建议
- 成本/稳定性建议
- 异常租户/异常 Key 标记

#### 3.21 观测与报警
- 成功率
- 延迟
- 错误分布
- Provider 健康排行
- 出口健康排行
- 模型热度

#### 3.22 运营能力
- 容量看板
- 成本统计
- 成本异常预警
- 风险日报
- 故障复盘草稿
- 峰值预测

#### 3.23 开发者工具链
- REST API
- 管理 API
- CLI
- SDK
- Webhook / Event Bus
- Mock / Replay / E2E / Chaos 测试

---

## 4. 自动化主线

### 4.1 账号自动化
- 自动导入账号
- 自动探活
- 自动分组与打标签
- 自动识别账号支持模型
- 自动识别风险状态
- 自动冷却 / 恢复 / 剔除
- 自动补充到目标池容量

### 4.2 模型自动化
- 自动拉取模型目录
- 自动发现模型上下线
- 自动更新模型画像
- 自动识别模型在哪些 Provider 可用
- 自动生成模型路由建议
- 自动维护 Fallback 链

### 4.3 出口自动化
- 自动探测出口可用性
- 自动打分出口质量
- 自动按地区匹配出口
- 自动切换劣化出口
- 自动隔离高风险出口
- 自动扩容 / 缩容出口池

### 4.4 浏览器 / Web Provider 自动化
- 自动检测网页登录态失效
- 自动 Session Warmup
- 自动 Profile 清理与重建
- 自动识别 DOM 变化
- 自动识别挑战页 / 异常页
- 自动分配更干净的 Browser Profile
- 自动切换备用 Web Provider

### 4.5 策略自动化
- 自动根据失败率调整池权重
- 自动根据成本调整模型优先级
- 自动生成降级策略
- 自动生成限流建议
- 自动标记异常租户 / 异常 Key

---

## 5. 安全主线

### 5.1 访问安全
- API Key 鉴权
- 签名请求（可选）
- 时间戳防重放
- IP 白名单 / 黑名单
- 路径级权限
- 方法级权限
- 敏感接口二次校验

### 5.2 管理面安全
- 管理面仅内网 / Tailscale 可达
- 管理操作审计
- 高危操作二次确认
- 导出类操作默认禁用
- 管理员分权

### 5.3 凭证安全
- Token / Cookie / Session 全部走 Secret Ref
- 过期预警
- 凭证轮换
- 最小权限访问
- 日志与导出禁明文

### 5.4 执行安全
- Bridge Worker 隔离
- Browser Worker 隔离
- Provider 执行上下文隔离
- 沙箱化
- 最小文件权限
- 最小网络权限

### 5.5 行为风险安全
- 请求行为异常检测
- 爆发式调用检测
- 可疑租户自动限流
- 可疑 Provider 自动降权
- 异常地区切换预警
- “高失败 + 高频率” 联合判断

### 5.6 配置安全
- 配置版本化
- 配置 Diff
- 策略审批
- 回滚快照
- 风险项扫描
- 不安全默认配置检测

---

## 6. 治理主线

### 6.1 策略治理
- 路由策略版本化
- 模型策略版本化
- 出口策略版本化
- 发布备注
- 灰度范围
- 回滚记录

### 6.2 组织治理
- Tenant / Project / Role
- RBAC
- 审批流
- 谁能改池
- 谁能改策略
- 谁能导入账号
- 谁能回滚

### 6.3 合规治理
- 审计留痕
- 敏感数据脱敏
- 数据留存周期
- 导出控制
- 最小披露
- 关键事件告警

### 6.4 变更治理
- 变更预检查
- Dry Run
- Canary 配置
- Shadow Routing
- 自动回滚阈值
- 发布窗口限制

---

## 7. 观测与可解释性主线

### 7.1 基础观测字段
- request id
- trace id
- tenant id
- api key id
- model
- provider class
- provider id
- account/session id
- egress id
- latency
- error type

### 7.2 路由解释
每次请求应能回答：
- 为什么选这个模型池
- 为什么没选另一个 Provider
- 为什么触发降级
- 为什么切到 Fallback
- 为什么被拒绝

### 7.3 健康解释
- 某账号为什么被冷却
- 某出口为什么被降权
- 某 Web Session 为什么被摘除
- 某模型为什么暂时不可用

### 7.4 趋势可视化
- 成功率趋势
- 延迟趋势
- 失败分布
- Provider 稳定性排行
- 出口健康排行
- 模型调用热度
- 租户用量趋势

---

## 8. 运营主线

### 8.1 资源运营
- 池容量看板
- 账号缺口预警
- Web Session 缺口预警
- 出口池压力预警
- 模型热点预警

### 8.2 成本运营
- 每模型成本
- 每租户成本
- 每 Provider 成本
- 每出口成本
- 成本异常波动预警
- 低成本替代建议

### 8.3 稳定性运营
- Provider SLA
- 池级 SLO
- 错误预算
- 故障时间线
- 恢复时间统计
- 高频事故画像

### 8.4 容量运营
- 峰值预测
- 并发阈值建议
- 资源自动扩缩建议
- 会话池预热建议
- 热模型容量储备建议

---

## 9. 开发者体验与平台化主线

### 9.1 平台接口
- REST API
- 管理 API
- CLI
- SDK
- Webhook
- Event Bus

### 9.2 测试能力
- Mock Upstream
- Mock Provider
- Mock Web Session
- E2E 测试
- 回归测试
- Chaos 测试

### 9.3 部署能力
- Docker Compose
- systemd
- 单机部署
- 分层部署
- 配置模板
- 一键健康检查

### 9.4 调试能力
- Trace Replay
- Request Replay
- 路由模拟器
- 策略 Dry Run
- 导出最小复现包

---

## 10. 智能化与未来扩展

### 10.1 智能化方向
- 智能路由建议
- 异常智能诊断
- 自适应策略
- 智能运营助手

### 10.2 多环境
- dev / staging / prod 分离
- 策略隔离
- 凭证隔离
- 审计隔离

### 10.3 多地域
- 多地域入口
- 多地域出口
- 地域级模型策略
- 灾备切换

### 10.4 多协议
- OpenAI 兼容
- Anthropic 兼容
- Gemini 风格兼容
- 内部统一协议

### 10.5 多执行器
- WS Bridge Executor
- API Executor
- Browser Executor
- Local Executor

---

## 11. 最终功能模块总表

建议补入以下十个最终模块：
1. 账号自动化管理模块
2. 模型自动发现与画像模块
3. 出口自动健康治理模块
4. Browser Session 自动运维模块
5. 策略自动优化模块
6. 配置治理与发布控制模块
7. 路由解释与复盘模块
8. 容量与成本运营模块
9. 异常检测与安全响应模块
10. 开发者工具链与回放模块

---

## 12. 核心数据模型总表

核心数据模型围绕八个中心对象群设计：**身份、资源、模型、调度、安全、状态、治理、运营**。

### 12.1 身份域
- Tenant
- Project
- ApiKey
- Role
- PermissionGrant

### 12.2 资源域
- Provider
- ProviderPool
- Account
- AccountPool
- WebSession
- BrowserProfile
- EgressProfile

### 12.3 模型域
- ModelCatalogEntry
- ModelAlias
- ModelCapabilityProfile
- ModelCostProfile
- ModelLatencyProfile
- ModelAvailabilityBinding

### 12.4 调度域
- RoutingPolicy
- RoutingDecision
- CandidateSet
- FallbackChain
- RateLimitPolicy
- QuotaPolicy

### 12.5 安全域
- SecretRef
- SecretVersion
- AccessPolicy
- NetworkBoundary
- RiskSignal
- RiskDecision

### 12.6 状态域
- HealthSnapshot
- PoolHealthSnapshot
- SessionHealthSnapshot
- EgressHealthSnapshot
- WarmupState
- CooldownState

### 12.7 治理域
- ConfigSnapshot
- ConfigDiff
- ChangePlan
- ReleaseRecord
- RollbackRecord
- AuditEvent

### 12.8 运营域
- UsageRecord
- CostRecord
- CapacitySnapshot
- AlertEvent
- SLORecord
- IncidentTimeline

---

## 13. 模块边界与调用关系

### 13.1 顶层模块
1. **Ingress Layer**
2. **OpenAI API Layer**
3. **Routing Layer**
4. **Execution Layer**
5. **State Layer**
6. **Governance Layer**
7. **Ops Layer**

### 13.2 顶层调用流
`Client -> OpenAI Compatible API -> Auth/Tenant -> Routing Engine -> Provider Pool -> Executor -> Upstream`

并行写入：
- Health State
- Audit
- Metrics
- Config Snapshot

### 13.3 六根主梁
1. Provider 抽象
2. Account / Session / Pool 抽象
3. Model + Routing 抽象
4. Egress / IP 抽象
5. Security / Governance 抽象
6. Automation / Ops 抽象

---

## 14. 风险清单与控制措施

### 14.1 技术风险
1. 上游协议变动 → Adapter 层 / Capability 探测 / 回归测试
2. 流式链路脆弱 → 流式状态机 / 首尾包监控 / Fail Closed
3. 动态路由过度复杂 → 路由分层 / Explainability / 先有限规则集

### 14.2 安全风险
4. 凭证泄漏 → Secret Ref / 脱敏 / 最小权限 / 禁明文导出
5. 管理面暴露 → 内网/Tailscale / 分权 / 二次确认 / 审计
6. Web Provider 高风险 → Profile 隔离 / Challenge 检测 / 自动冷却
7. IP 风险污染 → 入口出口分离 / 高风险池独立 / 健康评分

### 14.3 运营风险
8. 资源池耗尽 → 容量看板 / 阈值预警 / 自动 Warmup / Fallback 池
9. 成本失控 → Tenant 配额 / 模型预算 / 成本看板 / 替代建议
10. 故障不可解释 → Request/Trace ID / Decision 记录 / 时间线

### 14.4 产品风险
11. 目标过大长期不落地 → 分层路线图 / MVP 最小闭环 / 蓝图与交付分离
12. 功能堆积失控 → MUST / SHOULD / COULD / 每阶段只收口主干

---

## 15. 分层路线图

### L1：必须马上纳入蓝图
- 账号导入
- 模型动态分配
- Provider Pool
- Web Provider
- IP / Egress 安全
- 审计与解释

### L2：中期必须补齐
- Browser Session Pool
- 自动探活
- 冷却 / 熔断
- 配置版本化
- 发布治理
- 成本统计

### L3：后期高价值增强
- 策略自动优化
- 智能诊断
- 容量预测
- 自动日报
- 请求回放

### L4：平台化扩展
- 多租户强化
- 多协议
- 多地域
- 插件生态
- 控制台

---

## 16. 路线图收口

### V1
- OpenAI 兼容入口
- 基础 Provider 接入
- 鉴权
- Health / Models / Chat / Responses
- 基础错误归一化

### V2
- 账号导入
- 池化
- 动态路由
- Egress 安全

### V3
- Web Provider
- Session Pool
- Explainability
- 自动冷却 / 自动治理

### V4
- 治理、灰度、回滚、成本、运营、自动化增强

---

## 17. 最终收口判断

如果按当前方向继续完善，这个应用最有价值的终局，不是“更像 OpenAI 代理”，而是：

**一个安全、自动化、可运营的多上游 AI 调度平台。**

这是后续所有设计、实现、治理与运营动作的总收口。 
