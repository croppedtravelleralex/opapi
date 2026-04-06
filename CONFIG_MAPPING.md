# CONFIG_MAPPING.md

## 配置映射表（`auto_reg` -> Rust `api`）

本文档用于收口双项目之间的配置边界，避免把注册机内部运行配置直接污染到 Rust 网关项目。

---

## 一、总原则

### `auto_reg` 负责的配置

1. 邮箱服务配置
2. 验证码服务配置
3. 代理配置
4. 平台注册配置
5. 浏览器 / 指纹浏览器执行配置
6. 任务调度与批量注册配置

### Rust `api` 负责的配置

1. 网关监听配置
2. 网关鉴权配置
3. 上游路由配置
4. 账号池资源字段
5. 导入接口协议
6. 后续路由与状态消费配置

### 当前不建议同步到 Rust `api` 的内容

- `auto_reg` 的邮箱服务 API key
- 验证码服务 API key
- 浏览器内部 profile 细节
- 调度器内部运行参数
- 临时注册中间态上下文

---

## 二、`auto_reg` 环境变量分类

### A. 邮箱配置（保留在 `auto_reg`）

- `MOEMAIL_API_KEY`
- `SKYMAIL_API_KEY`
- `SKYMAIL_DOMAIN`
- `LUCKMAIL_API_KEY`
- `GPTMAIL_API_KEY`
- `CFWORKER_API_URL`
- `CFWORKER_DOMAIN`

### B. 验证码配置（保留在 `auto_reg`）

- `YESCAPTCHA_CLIENT_KEY`
- `LOCAL_SOLVER_URL`
- `SOLVER_BROWSER_TYPE`

### C. 代理配置（主归属 `auto_reg`）

- `PROXY_URL`

说明：
- 当前代理由注册机执行层消费
- 后续可把代理成功率 / 区域摘要同步到 Rust `api`
- 当前不建议把完整代理配置直接同步到 Rust `api`

### D. 外部系统集成配置（按用途拆分）

- `CPA_API_URL`
- `CPA_API_KEY`
- `SUB2API_URL`
- `SUB2API_KEY`
- `GROK2API_URL`
- `GROK2API_KEY`

说明：
- 如果这些系统只用于注册机上传 / 同步，则保留在 `auto_reg`
- 如果后续 Rust `api` 自己也要直连某系统，再在 Rust `api` 单独建配置，不共用 `.env`

### E. 注册机运行配置（保留在 `auto_reg`）

- `HOST`
- `PORT`
- `APP_RELOAD`
- `APP_CONDA_ENV`
- `LOG_LEVEL`
- `MAX_CONCURRENT_TASKS`
- `DEFAULT_RETRY_TIMES`

---

## 三、Rust `api` 当前配置范围

### A. 网关基础配置

- `HOST`
- `PORT`
- `API_TITLE`

### B. 网关鉴权配置

- `GATEWAY_API_KEYS`
- `GATEWAY_API_KEYS_FILE`

### C. 上游配置

- `UPSTREAM_BASE_URL`
- `UPSTREAM_API_KEY`
- `UPSTREAMS`
- `MODEL_UPSTREAM_MAP`
- `DEFAULT_MODELS`

### D. 账号池当前不需要配置化的部分

- SQLite 路径当前固定为 `data/gateway.db`
- 账号导入字段由接口协议决定，而不是 env 决定

---

## 四、账号导入字段映射

### `auto_reg` `AccountModel`

来自 `auto_reg/core/db.py`：
- `platform`
- `email`
- `password`
- `user_id`
- `region`
- `token`
- `status`
- `trial_end_time`
- `cashier_url`
- `extra_json`

### Rust `api` 导入接口字段

`POST /v1/accounts/import` / `POST /v1/accounts/status`：
- `platform`
- `email`
- `password`
- `user_id`
- `region`
- `token`
- `refresh_token`
- `status`
- `trial_end_time`
- `cashier_url`
- `extra_json`
- `provider`
- `label`
- `base_url`
- `model_scope`

### 推荐映射规则

| `auto_reg` 字段 | Rust `api` 字段 | 说明 |
|---|---|---|
| `platform` | `platform` | 原样保留 |
| `platform` | `provider` | 默认可等于平台名 |
| `email` | `email` | 原样保留 |
| `email` + `platform` | `label` | 默认组合成 `platform:email` |
| `password` | `password` | 原样保留 |
| `user_id` | `user_id` | 原样保留 |
| `region` | `region` | 原样保留 |
| `token` | `token` | 原样保留 |
| `status` | `status` | 原样保留 |
| `trial_end_time` | `trial_end_time` | 原样保留 |
| `cashier_url` | `cashier_url` | 原样保留 |
| `extra_json` | `extra_json` | 原样保留 |
| 平台/配置推断 | `base_url` | 由接入适配器补 |
| 平台/模型策略推断 | `model_scope` | 由接入适配器补 |
| 若存在 RT | `refresh_token` | `auto_reg` 后续若产出则带入 |

---

## 五、指纹浏览器配置映射建议

当前阶段不把指纹浏览器完整配置直接灌入 Rust `api`。

建议只在 `auto_reg` 抽一层执行器配置对象，例如：

- `browser_provider`
- `browser_api_url`
- `browser_api_key`
- `profile_id`
- `fingerprint_template`
- `proxy_binding_mode`
- `launch_args`

Rust `api` 只关心最终产物：
- 注册成功没有
- 账号状态是什么
- 该账号应导入哪个 provider / model_scope / base_url

---

## 六、当前最值动作

1. 先按本文档收口配置边界
2. 再写 `auto_reg -> Rust api` 适配器
3. 再抽象指纹浏览器执行器
4. 最后让导入账号逐步参与实际路由
