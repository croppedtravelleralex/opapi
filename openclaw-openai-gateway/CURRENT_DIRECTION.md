# 当前方向（openclaw-openai-gateway）

当前方向不是继续把它当“最小 demo 网关”往前糊，而是把它收口成 **多上游 AI 能力接入、路由调度、资源池治理、自动化运营平台** 的真实工程起点。

本阶段方向：

1. 先把 SQLite 从旁路副本推进成控制平面主读源
2. 先把 `provider_capabilities / model_availability` 真落表并 seed
3. 先让 routing 决策真正接到数据库里的 provider / capability / availability 数据
4. 再逐步引入 AccountPool、Governance、Audit Persistence、Automation 主梁

一句话：

**先从“最小真实 bridge skeleton”推进到“数据库驱动的最小控制平面 skeleton”。**
