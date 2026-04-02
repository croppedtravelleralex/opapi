# README.md

## 项目简介

这是一个围绕 **Sub2API / ChatGPT Business 账号池反代服务自动化** 的项目目录，目标是把现有方案文档逐步落地为一个真实可运行的 API 项目。

当前阶段已经从纯文档整理，推进到 **Rust 最小 API 骨架 + 单上游透传阶段**。

## 当前已有内容

- Sub2API 自动化架构设计
- Ubuntu + Docker 部署手册
- 项目入口/计划/状态/待办/进展文档骨架
- Rust 最小 API 骨架（`/healthz`、`/readyz`、`/v1/models`、`/v1/chat/completions`）
- `POST /v1/chat/completions` 已支持 **单上游 OpenAI 兼容接口透传**
- 配置设计文档 `CONFIG.md`
- 数据模型文档 `DATA_MODEL.md`

## 快速启动

```bash
cp .env.example .env
# 编辑 .env，填入 UPSTREAM_BASE_URL 和 UPSTREAM_API_KEY
cargo run
```

默认监听：`http://127.0.0.1:8088`

## 必填上游配置

```env
UPSTREAM_BASE_URL=https://your-upstream.example.com
UPSTREAM_API_KEY=sk-xxxx
```

## 快速验证

### 健康检查

```bash
curl http://127.0.0.1:8088/healthz
```

### 模型列表

```bash
curl http://127.0.0.1:8088/v1/models
```

### 聊天透传接口

```bash
curl -X POST http://127.0.0.1:8088/v1/chat/completions \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "gpt-5.4",
    "messages": [
      {"role": "user", "content": "hello"}
    ]
  }'
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
