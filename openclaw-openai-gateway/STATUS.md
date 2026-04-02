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
- smoke tests 已建立并通过（`9 passed / 0 failed`）
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
1. 先做母号 / 子号 / 空间 / 邀请 / 额度快照 / 池成员 / API Key 数据模型
2. 先做指纹浏览器 API 适配层
3. 先做“邀请 → 登录 → 验证空间 → 入池”主链路
4. 再做网页额度采集与入池/出池规则
5. 最后做独立 Key 输出与反代分发

---

## 当前阻塞
1. 还没有母号 / 子号 / 空间 / 池成员的正式数据模型
2. 还没有指纹浏览器 API 适配层
3. 还没有网页额度采集器
4. 还没有“子号入池 / 出池”状态机实现
5. 还没有对外独立 Key 管理层

---

## 运行形态判断
### 该砍
- GUI 桌面端
- 第三方 provider 主线
- 重前端运营后台

### 该加
- CLI 互动入口
- Worker / Scheduler
- 轻量 dashboard
- 审计查询接口
