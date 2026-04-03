# 当前任务（openclaw-openai-gateway）

当前任务：**把 Codex App / Web 额度反代主线正式落成第四段可运行骨架。**

本轮已完成：
1. 已把 admission 结果正式写回 `pool_members`
2. `collect` 接口现在会返回 `persisted_pool_member`，可直接验证入池结果
3. 已补 `pool_members` upsert 逻辑，支持同 child account 重复更新
4. 已补 smoke test，确认 `collect -> admission -> pool_members` 主链路打通
5. 已让控制面从“只会判断”推进到“会记录当前池状态”

下一轮继续推进：
1. 把 chat / responses 绑定到 `pool_members` 中可用来源
2. 落 `Codex Web` 多信号采集器
3. 给 `pool_members` 增加更多治理字段与冷却恢复逻辑
4. 补 pool routing 与边界测试

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
