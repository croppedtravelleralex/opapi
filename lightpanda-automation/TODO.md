# TODO.md

## P0

- [x] 建立项目核心文档（VISION / ROADMAP / STATUS / TODO / EXECUTION_LOG / RUN_STATE）
- [ ] 初始化 Rust 工程（Cargo）
- [ ] 设计任务数据模型（Task / Run / Artifact / Log）
- [ ] 设计 SQLite schema
- [ ] 定义 REST API 最小接口
- [ ] 实现内存任务队列
- [ ] 实现 fake runner
- [ ] 打通创建任务 -> 入队 -> 执行 -> 状态更新 -> 查询结果

## P1

- [ ] 增加任务取消 / 超时 / 重试机制
- [ ] 增加结构化日志
- [ ] 增加执行历史与审计字段
- [ ] 设计 runner trait / adapter interface
- [ ] 为 `lightpanda-io/browser` 预留适配层

## P2

- [ ] 增加并发控制
- [ ] 增加资源限制
- [ ] 增加 API 鉴权
- [ ] 增加基础监控指标
- [ ] 增加集成测试

## 待讨论

- [ ] 任务结果与 artifact 的落盘策略
- [ ] 截图 / HTML / console log 的存储方式
- [ ] 多租户/多用户隔离是否是近期目标
- [ ] 是否需要 webhook / callback 通知
