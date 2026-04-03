# 当前任务（openclaw-openai-gateway）

当前任务：**把 CodexExecutor 从来源感知 mock 推进为“真实会话执行入口骨架”，为后续接 Codex App / Web 真会话预留桥接层。**

本轮已完成：
1. 已新增 `CodexSessionBridge`，作为真实 Codex App / Web 会话执行入口骨架
2. 已给配置新增 `CODEX_SESSION_BRIDGE_MODE`，默认 `mock`，后续可切真实桥接模式
3. 已把 `CodexExecutor` 改成通过 session bridge 执行，而不是直接在 executor 内拼纯 mock 文本
4. 已保持 `source_id / source_page / child_account_id` 上下文继续贯穿到执行结果

下一轮继续推进：
1. 给 `CodexSessionBridge` 接真实 Codex App / Web 会话适配器
2. 落 `Codex Web` 多信号采集器（文本 + DOM + 快照）
3. 补来源上下文缺失 / 过期 / bridge 失败边界测试
4. 开始收 pool member 冷却 / 恢复 / 负载更新逻辑

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
