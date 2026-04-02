# 当前状态（openclaw-openai-gateway）

## 结论
当前工程已经不再是早期的 fake skeleton，而是进入了 **“最小真实平台骨架已形成”** 的阶段。

但这里必须区分两套进度口径：
- **骨架工程进度**：约 **84%–90%**
- **最终功能进度**：约 **25%–35%**

换句话说：
当前已经把 **主梁、控制平面雏形、最小 bridge、最小测试、最小 SQLite 持久化** 搭起来了，
但距离“完整多 Provider 智能接入与自动化运营平台”还远没完成。

## 已完成
- Rust 工程骨架已稳定
- `healthz` / `readyz` / `/v1/models` / `/v1/chat/completions` / `/v1/responses` 已落下
- Bearer API Key 中间件已接入
- request id 中间件已接入
- 最小 `Gateway Provider` bridge 已跑通
- `RoutingPolicy / RoutingDecision` 最小骨架已落下
- Explainability v1 已落下（`x-routing-explain`）
- Audit skeleton 已落下（`x-audit-action`）
- `ModelCatalog / ProviderPool` 最小实体已落下
- repository / in-memory persistence skeleton 已落下
- SQLite 已真实接入
- `model_catalog / providers` 已可建表并 seed
- `/v1/models` 已切到 SQLite-backed reads
- `/v1/providers` 已切到 SQLite-backed reads
- `provider_capabilities / model_availability` 已可建表并 seed
- audit 已落 SQLite（`audit_events`）
- capability / availability 已开始参与 routing 决策
- smoke tests 已建立并通过

## 当前主线
当前最优主线不是继续重复扩骨架，而是：

1. 把 `providers` 主读源切到 SQLite
2. 把 `provider_capabilities / model_availability` 真落表并 seed
3. 让 routing 决策开始真正使用 SQLite 数据
4. 再继续推进 governance / account pool / audit persistence

## 当前阻塞
1. 仍缺真正的多 Provider 真实行为
2. 仍缺 capability / availability 真正参与路由
3. 仍缺 account / session pool
4. 仍缺 governance / audit / config snapshot 的完整持久化
5. 流式、embeddings、images、audio 仍未开始

## 当前判断
该项目现在最值动作是：**把“SQLite 作为旁路持久化”推进到“SQLite 成为控制平面真实数据底座”。**
