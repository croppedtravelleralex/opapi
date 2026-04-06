# INTEGRATION_PLAN.md

## 注册机与网关接入边界（当前定版）

### 角色划分

#### 1. `auto_reg`
定位：**注册机 / 账号供给侧主项目**

负责：
- 平台注册流程
- 邮箱服务接入
- 验证码服务接入
- 代理与浏览器执行
- 任务调度
- 注册结果产出

#### 2. Rust `api`
定位：**账号池 / 路由池 / OpenAI 兼容网关**

负责：
- 账号导入
- 账号状态存储
- 账号资源可观测
- 后续路由决策
- 对外 API 调用入口

---

## 配置映射边界

当前建议把配置分成 5 类：

1. **邮箱配置**
   - 归属：`auto_reg`
   - 示例：MoeMail / SkyMail / CF Worker / 自建邮箱

2. **验证码配置**
   - 归属：`auto_reg`
   - 示例：YesCaptcha / local solver

3. **代理配置**
   - 归属：`auto_reg`
   - 后续可同步摘要状态到 Rust `api`

4. **平台配置**
   - 归属：`auto_reg`
   - 示例：ChatGPT / Cursor / Grok / Kiro 注册参数

5. **浏览器配置**
   - 当前归属：`auto_reg`
   - 后续接入指纹浏览器 API 时，优先走统一执行器抽象

Rust `api` 当前只保存与路由/账号池直接相关的资源字段，不保存注册机内部所有运行配置。

---

## 指纹浏览器执行器抽象

当前建议在注册机侧抽一层统一执行器接口：

### 抽象目标

让平台插件不要直接依赖：
- Playwright
- Camoufox
- 某一个固定浏览器内核
- 某一个固定指纹浏览器 API

### 建议抽象层

- `browser_executor`
- `browser_profile`
- `browser_session`
- `browser_navigation`
- `browser_identity`

### 最小能力

1. 创建浏览器实例
2. 加载/创建 profile
3. 注入代理
4. 注入指纹参数
5. 打开页面
6. 执行点击 / 输入 / 等待 / 获取验证码页面状态
7. 导出执行结果与错误上下文

---

## 注册结果回传协议

当前 Rust `api` 已提供：

- `POST /v1/accounts/import`
- `POST /v1/accounts/status`

### 推荐导入字段

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

### 推荐流程

1. `auto_reg` 注册成功
2. 标准化账号结果对象
3. 调用 `/v1/accounts/import`
4. 后续状态变化调用 `/v1/accounts/status`
5. Rust `api` 作为账号资源真相源供路由层消费

---

## 当前不建议做的事

1. 不建议直接共用 `auto_reg` 的 SQLite 作为 Rust 网关主库
2. 不建议把 `auto_reg` 前端直接并入 Rust `api` 项目
3. 不建议让指纹浏览器 API 直接散落进每个平台插件内部
4. 不建议现在就让所有导入账号直接参与复杂动态路由

---

## 当前最值主线

1. 正式确认 `auto_reg` = 注册机主项目
2. 收口配置映射
3. 抽象指纹浏览器执行器
4. 稳定注册结果回传协议
5. 再逐步让导入账号参与实际路由
