# TODO.md

## P0

- [x] 建立 `AI.md`
- [x] 建立 `README.md`
- [x] 建立 `PLAN.md`
- [x] 建立 `FEATURES.md`
- [x] 建立 `CURRENT_DIRECTION.md`
- [x] 建立 `CURRENT_TASK.md`
- [x] 建立 `STATUS.md`
- [x] 建立 `PROGRESS.md`
- [x] 建立 `TODO.md`
- [x] 设计并落下 Rust 最小代码骨架
- [x] 跑通最小启动验证与 Bearer 鉴权验证
- [x] 收口多上游 `/v1` 路径语义
- [x] 跑通双上游 smoke 验证
- [x] 增强 `/v1/models` 路由视图与状态视图
- [x] 补上游错误映射与归一化
- [x] 落下 SQLite 最小账号池 store
- [x] 提供 `/v1/accounts` 只读接口
- [x] 提供 `/v1/accounts/import` 与 `/v1/accounts/status`
- [x] 将规划正式重排为：注册机 = `auto_reg`，Rust `api` = 账号池 / 路由池 / 网关
- [ ] 同步清理 `STATUS / TODO / PROGRESS / README` 与当前真实状态
- [ ] 评估并整理 `data/gateway.db` 是否继续纳入版本库
- [ ] 收口 `task_plan.md / findings.md / progress.md` 等工作态文件

## P1

- [ ] 设计指纹浏览器执行器抽象层
- [ ] 输出双项目配置映射表
- [ ] 写 `auto_reg -> Rust api` 对接适配器
- [ ] 让导入账号逐步参与实际路由
- [ ] 继续补账号状态 / 健康状态 / fallback 基础

## P2

- [ ] 评估动态路由模式的最小落地路径
- [ ] 评估监控告警与审计模块前置程度
- [ ] 评估注册机任务调度与网关资源池的长期协作方式
