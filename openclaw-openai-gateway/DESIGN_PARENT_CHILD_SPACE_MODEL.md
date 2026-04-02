# DESIGN_PARENT_CHILD_SPACE_MODEL.md

## 核心规则

### 母号 / 子号约束
- 一个母号对应一个空间
- 一个母号可以邀请多个子号
- 一个子号只能加入一个母号
- 子号是已有账号，不是临时注册的新号
- 母号不进入额度池
- 子号验证完成后才有资格进入额度池

---

## 关键实体

### 1. parent_accounts
建议字段：
- `id`
- `email`
- `space_name`
- `status`
- `fingerprint_profile_id`
- `invite_enabled`
- `risk_level`
- `last_login_at`

状态建议：
- `new`
- `login_verified`
- `space_ready`
- `invite_enabled`
- `risk_hold`
- `disabled`

### 2. child_accounts
建议字段：
- `id`
- `email`
- `parent_account_id`
- `status`
- `space_verified`
- `pool_status`
- `risk_level`
- `fingerprint_profile_id`
- `last_login_at`

状态建议：
- `new`
- `invited`
- `login_pending`
- `space_verifying`
- `verified`
- `quota_observing`
- `eligible`
- `in_pool`
- `cooling`
- `suspended`
- `dead`

### 3. space_memberships
建议字段：
- `id`
- `parent_account_id`
- `child_account_id`
- `joined`
- `verified`
- `verified_at`

### 4. invite_tasks
建议字段：
- `id`
- `parent_account_id`
- `child_account_id`
- `status`
- `sent_at`
- `accepted_at`
- `error_reason`

---

## 关键流程

### 流程 1：导入母号
1. 导入母号
2. 指纹浏览器登录
3. 验证空间管理能力
4. 建立 `parent_accounts`

### 流程 2：邀请子号
1. 选择母号
2. 选择目标子号
3. 校验子号是否已绑定其他母号
4. 发送邀请
5. 记录 `invite_tasks`

### 流程 3：子号验空间
1. 子号在指纹浏览器登录
2. 进入目标空间
3. 验证归属关系
4. 标记 `verified`

### 流程 4：进入额度池
1. 拉取额度页
2. 检查风险状态
3. 通过后转 `eligible -> in_pool`
