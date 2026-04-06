# CURRENT_TASK.md

## 当前任务

当前任务已经不是继续搭项目骨架，而是把 **最小可用网关闭环** 真正收口：

1. 跑通 **health / models / chat** 的本地联调
2. 验证 **Bearer 鉴权** 在真实请求中生效
3. 收口 **静态多上游映射 + `/v1` 路径拼接语义**
4. 补齐一轮可复用的 smoke 验证脚本与项目状态文档

---

## 当前阶段交付物

- [x] 建立项目入口文档 `AI.md`
- [x] 建立 `README.md`
- [x] 建立 `PLAN.md`
- [x] 建立 `FEATURES.md`
- [x] 建立 `CURRENT_DIRECTION.md`
- [x] 建立 `CURRENT_TASK.md`
- [x] 建立 `STATUS.md`
- [x] 建立 `PROGRESS.md`
- [x] 建立 `TODO.md`
- [x] 初始化 Git 仓库
- [x] 设计并落下最小代码骨架
- [x] 明确 V1 API 最小闭环
- [x] 跑通最小服务启动与健康检查
- [x] 跑通 Bearer 鉴权验证
- [x] 跑通首条 chat 真实透传链路
- [x] 落下静态多上游 smoke 验证脚本
- [ ] 收口 `/v1` 语义并完成双上游验证通过
- [ ] 同步 README / STATUS / TODO / PROGRESS 到当前真实状态

---

## 下一步优先级

### P0
1. 修正并统一多上游 `/v1` 路径拼接策略
2. 跑通双上游 smoke 验证
3. 同步状态文档，避免文档落后于代码
4. 收口当前未提交改动并提交 Git

### P1
5. 让 `/v1/models` 支持更真实的路由侧视图
6. 继续补错误映射与上游异常处理
7. 整理最小部署与验证流程
