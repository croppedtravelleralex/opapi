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
- [x] 补 `.gitignore`
- [x] 补 `CONFIG.md` 与 `DATA_MODEL.md`
- [x] 补 `README` 启动说明与 curl 示例
- [x] 补 `/v1/chat/completions` 占位接口
- [x] 清理 Git 仓库中误提交的 `target/`
- [x] 跑通 `cargo run` 最小启动验证
- [x] 明确 V1 真实转发闭环
- [x] 跑通 Bearer 鉴权真实验证
- [x] 跑通单条 chat 透传真实验证
- [ ] 统一多上游 `/v1` 路径拼接语义
- [ ] 跑通双上游 smoke 验证
- [ ] 收口 README / STATUS / TODO / PROGRESS 同步

## P1

- [x] 设计鉴权入口
- [x] 设计上游配置与路由方式
- [x] 设计 health / models 之外的真实转发接口
- [x] 设计最小部署验证流程
- [ ] 让 `/v1/models` 具备更真实的路由视图
- [ ] 继续补错误映射与上游异常处理

## P2

- [ ] 评估是否拆分为真正独立仓库
- [ ] 评估自动化 OAuth/健康检查脚本落地顺序
- [ ] 评估监控告警与审计模块前置程度
