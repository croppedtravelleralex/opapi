# 当前任务（openclaw-openai-gateway）

当前任务：**把“注册机 + 邮箱池 + quota pool + automation target”写进文档，并核查本机运行环境 / 运行条件 / 安全整改项。**

本轮已完成：
1. 已把自动注册机推进到 registration task queue + verification + dead-letter recover
2. 已把邮箱池推进到 import / poll / overview / expand / auto tiering
3. 已把 automation target 推进到 discover / try / attempt history
4. 已补轻 dashboard / ops overview / scheduler tick 三个运维入口
5. 已明确 dashboard 访问方式：**SSH 远程转发，不直接公网暴露**
6. 已完成本机只读运行环境核查：OpenClaw 状态、security audit、update status、监听端口、磁盘、内存、工具链、项目测试
7. 已确认项目代码运行正常，但主机安全条件需要整改

下一轮继续推进：
1. 按风险优先级整改主机与 OpenClaw 运行条件
2. 做邮箱池压力均衡器 / 健康看板 / 自动恢复
3. 把 scheduler 从手动 tick 推进到常驻调度
4. 继续把注册机接近真实指纹浏览器执行入口
5. 收敛剩余 dead-code / repo warning

当前最关键整改项：
- `openclaw security audit --deep` 中 2 个 critical 插件问题
- 主机缺少防火墙层
- `cupsd:631` 对外监听是否必要需确认
- OpenClaw 配置项 `autoAllowSkills / strictInlineEval / trustedProxies` 需要收紧

本轮不追求：
- GUI
- 重 dashboard
- provider 市场化扩张
