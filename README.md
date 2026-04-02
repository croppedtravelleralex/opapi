# README.md

## 项目简介

这是一个围绕 **Sub2API / ChatGPT Business 账号池反代服务自动化** 的项目目录，目标是把现有方案文档逐步落地为一个真实可运行的 API 项目。

当前阶段已经从纯文档整理，推进到 **Rust 最小 API 骨架 + 静态多上游路由阶段**。

## 当前已有内容

- Sub2API 自动化架构设计
- Ubuntu + Docker 部署手册
- 项目入口/计划/状态/待办/进展文档骨架
- Rust 最小 API 骨架（`/healthz`、`/readyz`、`/v1/models`、`/v1/chat/completions`）
- `POST /v1/chat/completions` 已支持 **单上游透传** 与 **静态多上游模型映射预留**
- 最小网关鉴权设计（`GATEWAY_API_KEYS` / `GATEWAY_API_KEYS_FILE`）
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

## 静态多上游配置

```env
UPSTREAMS=oa|https://api.openai.com|sk-oa;iflow|https://example-iflow.test|sk-iflow
MODEL_UPSTREAM_MAP=gpt-5.4=oa,qwen3-max=iflow
```

## 快速验证

### 健康检查（免鉴权）

```bash
curl http://127.0.0.1:8088/healthz
```

### 模型列表（需要 Bearer）

```bash
curl http://127.0.0.1:8088/v1/models \
  -H 'Authorization: Bearer sk-local-demo'
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

```bash
python3 scripts/mock_upstream.py 19091 mock-a &
python3 scripts/mock_upstream.py 19092 mock-b &

PORT=8088 \
GATEWAY_API_KEYS=sk-local-demo \
UPSTREAMS='a|http://127.0.0.1:19091|dummy;b|http://127.0.0.1:19092|dummy' \
MODEL_UPSTREAM_MAP='gpt-5.4=a,qwen3-max=b' \
cargo run

./scripts/smoke.sh http://127.0.0.1:8088 sk-local-demo gpt-5.4 qwen3-max
```

## 当前目标

优先把这个目录从“文档堆”推进成“有统一入口、有清晰主线、可开始编码和验证的项目目录”。

## 当前建议阅读顺序

1. `AI.md`
2. `PLAN.md`
3. `FEATURES.md`
4. `STATUS.md`
5. `PROGRESS.md`
6. 原始方案文档
7. `src/`
