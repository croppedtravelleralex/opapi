# README.md

## 项目简介

这是一个围绕 **Sub2API / ChatGPT Business 账号池反代服务自动化** 的项目目录，目标是把现有方案文档逐步落地为一个真实可运行的 API 项目。

当前阶段已经从纯文档整理，推进到 **Rust 最小可用网关 + 鉴权验证 + 静态多上游 smoke 验证 + SQLite 最小账号池骨架阶段**。

## 当前已有内容

- Sub2API 自动化架构设计
- Ubuntu + Docker 部署手册
- 项目入口/计划/状态/待办/进展文档骨架
- Rust 最小 API 骨架（`/healthz`、`/readyz`、`/v1/models`、`/v1/chat/completions`、`/v1/accounts`）
- `POST /v1/chat/completions` 已支持 **单上游透传** 与 **静态多上游模型映射**
- `/v1/*` 已接入最小 Bearer 鉴权，并已通过真实 401/200 联调
- 多上游 `/v1` 拼接语义已统一，支持自动推断与显式覆盖
- `/v1/models` 已可返回模型对应的上游、目标端点与基础状态视图
- 已接入本地 SQLite，启动时会初始化 `data/gateway.db`
- 已落最小账号池 store，并提供 `/v1/accounts` 只读视图
- 配置设计文档 `CONFIG.md`
- 数据模型文档 `DATA_MODEL.md`
- 路由演进文档 `ROUTING_PLAN.md`
- Docker / Compose / smoke / mock upstream 工具链

## 快速启动

```bash
cp .env.example .env
# 编辑 .env，填入网关配置
cargo run
```

默认监听：`http://127.0.0.1:8088`

首次启动后会自动创建：

```bash
data/gateway.db
```

## 鉴权配置

```env
GATEWAY_API_KEYS=sk-local-demo
# 或者
GATEWAY_API_KEYS_FILE=keys.example.txt
```

## 单上游配置

```env
UPSTREAM_BASE_URL=https://your-upstream.example.com
UPSTREAM_API_KEY=sk-xxxx
```

默认规则：
- 如果 `UPSTREAM_BASE_URL` **不以 `/v1` 结尾**，网关会自动补 `/v1/chat/completions`
- 如果 `UPSTREAM_BASE_URL` **已经以 `/v1` 结尾**，网关会直接拼到 `/chat/completions`

## 静态多上游配置

```env
UPSTREAMS=oa|https://api.openai.com|sk-oa;iflow|https://example-iflow.test|sk-iflow
MODEL_UPSTREAM_MAP=gpt-5.4=oa,qwen3-max=iflow
```

可选第 4 段用于覆盖 `/v1` 规则：

```env
UPSTREAMS=oa|https://api.openai.com|sk-oa|append-v1;iflow|https://example-iflow.test/v1|sk-iflow|no-append-v1
```

规则说明：
- `append-v1`：强制拼接成 `.../v1/chat/completions`
- `no-append-v1`：强制拼接成 `.../chat/completions`
- 不写第 4 段：按 `base_url` 是否以 `/v1` 结尾自动判断

## 最小账号池说明

当前版本已接入本地 SQLite 账号池骨架，启动时会自动 seed 两条演示数据。

### 查看账号池（需要 Bearer）

```bash
curl http://127.0.0.1:8088/v1/accounts \
  -H 'Authorization: Bearer sk-local-demo'
```

### 查看模型路由视图（需要 Bearer）

```bash
curl http://127.0.0.1:8088/v1/models \
  -H 'Authorization: Bearer sk-local-demo'
```

## 快速验证

### 健康检查（免鉴权）

```bash
curl http://127.0.0.1:8088/healthz
```

### 聊天透传接口（需要 Bearer）

```bash
curl -X POST http://127.0.0.1:8088/v1/chat/completions \
  -H 'Authorization: Bearer sk-local-demo' \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "gpt-5.4",
    "messages": [
      {"role": "user", "content": "hello"}
    ]
  }'
```

### 本地 mock 双上游联调

直接运行：

```bash
./scripts/smoke_multi_upstream.sh
```

脚本会自动完成：
- 启动两个 mock upstream
- 启动本地网关
- 验证 `/healthz`
- 验证 `/v1/models` 的 401/200 鉴权行为
- 验证 `gpt-5.4` 与 `qwen3-max` 分别路由到不同 mock upstream

## 当前目标

优先把这个目录从“文档堆”推进成“**最小可用、可验证、可继续扩展，并开始具备资源层雏形的网关项目**”。

## 当前建议阅读顺序

1. `AI.md`
2. `PLAN.md`
3. `FEATURES.md`
4. `STATUS.md`
5. `PROGRESS.md`
6. `ROUTING_PLAN.md`
7. `DATA_MODEL.md`
8. `src/`
