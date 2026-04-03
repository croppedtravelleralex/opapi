# 当前任务（openclaw-openai-gateway）

当前任务：**把项目收成最适合本机长期运行的单机轻服务，只保留 Codex 额度主链。**

本轮已完成：
1. 已从主运行面移除 governance 路由
2. 已把项目定位重写为：`SQLite + API + Codex 主链 + 最小调度`
3. 已明确当前不追求 GUI / 重 dashboard / 第三方 provider 扩展
4. 已把主服务边界收口到 health / models / providers / codex / chat / responses

下一轮继续推进：
1. 冻结或删除 governance 相关模块与文档
2. 冻结第三方 provider 扩展相关代码路径
3. 继续只做 `codex-app` 真实会话适配器
4. 后续按需要补轻量 scheduler，而不是扩重控制面

本轮不追求：
- GUI
- 重 dashboard
- 治理平台
- 第三方 provider 市场化能力
