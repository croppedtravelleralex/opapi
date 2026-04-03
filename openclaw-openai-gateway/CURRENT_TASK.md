# 当前任务（openclaw-openai-gateway）

当前任务：**把 Codex App / Web 额度反代主线正式落成第三段可运行骨架。**

本轮已完成：
1. 已建立“额度观测结果 → 准入判定”的最小 admission 层
2. `collect` 接口现在会直接返回 `pool_status / admission_level / weight / reasons`
3. 已落最小三档规则：`green / yellow / red`
4. 已落最小冷却逻辑：读取失败或额度过低会进入 `cooling`
5. 已补 smoke tests，覆盖健康额度 / 低额度 / 读取失败三类判定

下一轮继续推进：
1. 把 admission 结果真正写入 `pool_members`
2. 建立按可用额度来源做最小路由的执行入口
3. 落 `Codex Web` 多信号采集器
4. 补 parser 与 admission 的边界测试

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
