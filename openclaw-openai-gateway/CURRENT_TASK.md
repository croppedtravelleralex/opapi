# 当前任务（openclaw-openai-gateway）

当前任务：**把 Codex App / Web 额度反代主线正式推进到最小数据面路由。**

本轮已完成：
1. 已新增 `pool_router`，可从 `pool_members` 中挑选当前最优 active 来源
2. 已让 `chat / responses` 在真正转发前先检查额度池里是否有可用来源
3. 已新增 `no_healthy_pool_member` 错误分支，避免空池时继续盲转发
4. 已把选中的池成员信息通过响应头回传，便于后续审计与排障
5. 已开始把控制面能力接到最小数据面入口

下一轮继续推进：
1. 让请求真正按池成员绑定到具体额度来源执行
2. 落 `Codex Web` 多信号采集器
3. 补 pool 路由优先级 / 空池 / 冷却边界测试
4. 给 pool member 增加更真实的 load / cooldown 更新逻辑

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
