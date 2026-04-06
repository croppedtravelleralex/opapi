# PROGRESS.md

`api` 项目进展记录（面向老板 / 甲方版）。

说明：
- 只记录已经完成的项目级成果
- 不把方案目标写成已实现能力
- 目标是让不看代码的人也能看懂现在到哪了

---

## 功能进展记录

- **2026年04月02日 20时00分00秒** 实现了**项目入口与项目规则收口**功能，为该目录建立了 `AI / PLAN / FEATURES / STATUS / TODO / PROGRESS / CURRENT_* / README` 文档骨架。
- **2026年04月02日 20时27分00秒** 实现了**Rust 最小 API 骨架**功能，新增 `Cargo.toml`、`.env.example` 和 `src/` 结构，并落下 `/healthz`、`/readyz`、`/v1/models` 三个基础接口。
- **2026年04月02日 20时36分00秒** 实现了**聊天补位接口与工程收口增强**功能，补上 `/v1/chat/completions` 占位接口、`.gitignore`、`CONFIG.md`、`DATA_MODEL.md`，并补充了启动说明与 curl 验证示例。
- **2026年04月02日 20时42分00秒** 实现了**单上游聊天透传能力**，将 `/v1/chat/completions` 从 placeholder 升级为可透传到单个 OpenAI 兼容上游的最小真实链路，并补入 `UPSTREAM_BASE_URL / UPSTREAM_API_KEY` 配置入口。
- **2026年04月02日 21时06分00秒** 实现了**最小网关鉴权与路由演进规划**，补入 `GATEWAY_API_KEYS`、最小 Bearer 鉴权中间件设计，并新增 `ROUTING_PLAN.md` 规划单上游到多上游的演进边界。
- **2026年04月06日 14时28分00秒** 实现了**最小联调验证闭环**，确认 `cargo run` 可稳定启动，`/healthz` 可正常返回，且 `/v1/models` 在未带 Bearer 时返回 `401`、带 Bearer 时返回 `200`。
- **2026年04月06日 14时29分00秒** 实现了**首条多上游真实转发验证**，确认 `gpt-5.4` 可按静态模型映射路由到 mock upstream 并拿到真实 completion 返回，同时定位出多上游 `/v1` 拼接语义不一致的真实问题。
- **2026年04月06日 15时13分00秒** 实现了**多上游 smoke 验证脚本骨架**，新增 `scripts/smoke_multi_upstream.sh`，用于联动 mock upstream、网关启动、鉴权检查与双模型转发验证。

---

## 当前阶段一句话总结

**截至 2026年04月06日，本目录已经从“纯方案文档目录”升级为“具备最小 Rust 网关骨架、Bearer 鉴权、单链路真实转发验证、静态多上游配置能力和 smoke 验证脚本雏形”的最小可用网关起步目录。**
