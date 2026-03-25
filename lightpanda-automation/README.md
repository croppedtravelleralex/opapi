# lightpanda-automation

高性能浏览器自动化系统，运行在 Ubuntu 上。

## 项目目标

构建一个面向自动化任务执行的浏览器系统，当前采用：

- Rust
- SQLite
- REST API
- 内存任务队列
- fake runner

后续将接入 `lightpanda-io/browser` 作为真实浏览器执行引擎。

## 当前阶段

当前处于项目骨架与架构定义阶段。

## 关键文档

- `VISION.md` — 最终效果与最终功能定义
- `ROADMAP.md` — 过去 / 现在 / 未来的滚动路线图
- `STATUS.md` — 当前状态、风险、下一步
- `TODO.md` — 任务分层清单
- `EXECUTION_LOG.md` — 每轮执行记录
- `RUN_STATE.json` — 自动推进的轮次状态
- `AUTONOMY_PLAN.md` — 周期执行规则
- `CURRENT_DIRECTION.md` — 当前阶段方向说明

## 目录建议

- `src/` — Rust 主程序与模块
- `migrations/` — SQLite schema / 迁移
- `docs/` — 架构文档、接口说明
- `scripts/` — 开发辅助脚本
- `examples/` — 示例请求与样例任务

## 后续目标

1. 完成最小可运行后端骨架
2. 打通任务创建 / 入队 / 执行 / 状态更新链路
3. 用 fake runner 跑通端到端流程
4. 接入真实浏览器引擎
5. 补齐观测、重试、资源隔离与稳定性能力
