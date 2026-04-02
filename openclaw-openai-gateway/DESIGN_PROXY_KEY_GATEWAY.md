# DESIGN_PROXY_KEY_GATEWAY.md

## 目标
将内部额度池转换为对外可分配的独立 API Key 服务。

---

## 核心要求
- 按用户分配独立 Key
- 每个 Key 可以独立限额与审计
- 内部调度使用池内子号
- 对外继续暴露 OpenAI 兼容接口

---

## 数据模型
### proxy_api_keys
- `id`
- `label`
- `hashed_key`
- `owner`
- `status`
- `rate_limit`
- `quota_limit`
- `allowed_models`

### proxy_usage_logs
- `id`
- `proxy_api_key_id`
- `child_account_id`
- `request_type`
- `model`
- `request_count_delta`
- `token_delta`
- `message_delta`
- `status`
- `created_at`

---

## 请求处理流程
1. 外部请求进入网关
2. 校验外部分配的 API Key
3. 根据可用额度池挑选一个可用子号
4. 在对应安全会话中执行
5. 返回结果
6. 记录消耗与审计
7. 必要时调整池状态

---

## 产品形态建议
### 必须有
- CLI 生成/禁用/查看 API Key
- dashboard 查看 Key 使用情况
- 审计查询

### 暂不需要
- GUI 桌面客户端
- 富交互可视化编辑器
- provider 市场
