# openclaw-openai-gateway

## 结论
`openclaw-openai-gateway` 当前不再以“第三方 provider 接入”作为主线，而是转向：

**先把真实额度反代做成，再把额度组织成池，最后对外提供独立 API Key。**

当前优先额度来源：
1. `Codex App`
2. `Web` 网页端

明确暂缓：
- 第三方 API Key + BaseURL provider 扩展
- GUI 桌面端
- 重前端运营平台

---

## 当前产品形态
项目定位为：

**额度池驱动的 OpenAI 兼容代理平台**

核心链路：
- 母号管理一个空间
- 邀请已有子号
- 子号在指纹浏览器中登录并验证空间
- 网页端读取 5h / 7d 额度及 request / token / message
- 合格子号进入额度池
- 对外按用户分配独立 API Key 反代

---

## 已有底座
- Rust API Server
- `healthz` / `readyz`
- `/v1/models` / `/v1/chat/completions` / `/v1/responses`
- Bearer API Key middleware
- request id middleware
- Gateway Provider 最小 bridge
- SQLite 控制平面骨架
- `/v1/models` / `/v1/providers` SQLite-backed reads
- `provider_capabilities / model_availability` 建表与 seed
- audit 持久化到 SQLite
- governance skeleton
- smoke tests（当前通过）

---

## 新增设计文档
- `VISION_QUOTA_PROXY.md`
- `DESIGN_PARENT_CHILD_SPACE_MODEL.md`
- `DESIGN_QUOTA_OBSERVATION.md`
- `DESIGN_POOL_ADMISSION_AND_EJECTION.md`
- `DESIGN_FINGERPRINT_BROWSER_ADAPTER.md`
- `DESIGN_PROXY_KEY_GATEWAY.md`
- `RUNTIME_AND_PRODUCT_SHAPE_REVIEW.md`

---

## 当前最优实现顺序
1. 落母号 / 子号 / 空间 / 邀请 / 额度快照 / 池成员 / 外部 API Key 表结构
2. 落指纹浏览器 API 适配层
3. 落“邀请 → 登录 → 验证空间 → 进入额度池”主链路
4. 落网页额度采集与入池/出池规则
5. 落对外独立 API Key 与反代分发

---

## 运行形态建议
### 应该保留
- API Server
- CLI 互动入口
- Worker / Scheduler
- 轻量 dashboard

### 应该砍掉或延后
- GUI 桌面端
- provider 花活
- 重前端后台

---

## Run
```bash
cp .env.example .env
cargo run
```
