# 当前任务（openclaw-openai-gateway）

当前任务：**把 Codex App / Web 额度反代主线正式落成第二段可运行骨架。**

本轮已完成：
1. 已把 `Codex App` 额度采集器骨架落到代码（collector + parser）
2. 已新增 `/v1/codex/quota/collect`，可直接提交页面文本并生成 `quota_snapshots`
3. 已实现最小字段解析：`5h / 7d / requests / tokens / messages`
4. 已把采集结果直接落 SQLite，为后续入池判断做准备
5. 已补 smoke tests，确认“可解析 / 解析失败”两条分支都通过

下一轮继续推进：
1. 落 `Codex Web` DOM/文本/快照多信号采集器
2. 建立“观测结果 → admission_level / pool_status”判定层
3. 开始把 chat / responses 请求绑定到真实可用额度来源
4. 补 parser 的页面变体兼容与错误分类

本轮不追求：
- GUI
- 第三方 provider 扩展
- 重前端后台
