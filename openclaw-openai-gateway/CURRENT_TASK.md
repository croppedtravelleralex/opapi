# 当前任务（openclaw-openai-gateway）

当前任务：**把 Codex App / Web 额度反代主线正式落成第一段可运行骨架。**

本轮已完成：
1. 已把 `Codex App / Web` 额度来源抽象落到代码（`codex_quota_source`）
2. 已新增 `/v1/codex/quota-sources`，可直接查看当前额度来源清单
3. 已新增 `/v1/codex/quota-overview`，可直接汇总 `quota_snapshots` 观测统计
4. 已把 `codex.app / codex.web` 作为 Web 类 provider 注入当前 provider pool
5. 已补 smoke tests，确认新接口可用且统计逻辑通过

下一轮继续推进：
1. 落 `Codex App` 真正的额度采集器接口
2. 落 `Codex Web` 真正的额度采集器接口
3. 建立统一“观测结果 → 可用性状态 → 入池判断”转换层
4. 开始收反代执行入口与额度来源绑定关系

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
