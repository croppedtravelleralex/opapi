# CONFIG.md

## 配置结构设计（V1）

当前 V1 先只保留最小可运行配置，不急着一上来做成超复杂大系统。

---

## 基础服务配置

- `HOST`：监听地址，默认 `0.0.0.0`
- `PORT`：监听端口，默认 `8088`
- `API_TITLE`：服务名称，默认 `sub2api-gateway`
- `DEFAULT_MODELS`：默认暴露的模型列表，逗号分隔

---

## 下一阶段预留配置

### 上游路由
- `UPSTREAM_BASE_URL`
- `UPSTREAM_API_KEY`
- `UPSTREAM_TIMEOUT_MS`

### 鉴权
- `GATEWAY_API_KEYS`
- `ADMIN_TOKEN`

### 存储
- `DATABASE_URL`
- `REDIS_URL`

### 调度
- `ROUTING_MODE`
- `DEFAULT_PROVIDER`
- `MODEL_PROVIDER_MAP`

---

## 当前原则

1. 先保证最小配置能启动
2. 再补上游转发配置
3. 再补账号池 / 存储 / 调度相关配置
4. 不提前把配置复杂化
