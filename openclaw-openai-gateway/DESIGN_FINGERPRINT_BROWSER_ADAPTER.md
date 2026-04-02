# DESIGN_FINGERPRINT_BROWSER_ADAPTER.md

## 目标
对接用户本地提供的指纹浏览器 API，统一执行所有高风险动作。

---

## 原则
以下动作必须经过指纹浏览器：
- 母号登录
- 子号登录
- 空间邀请
- 接受邀请 / 验证空间
- 额度页访问
- 使用量页面访问
- 登录态校验

---

## 为什么必须有这一层
- 母号与子号必须环境隔离
- 不共享 cookie / storage / profile
- 高风险动作必须可审计
- 后续可对接 `lightpanda automation` 风格 API

---

## 建议接口

### Profile 管理
- `profile.create`
- `profile.open`
- `profile.close`
- `profile.recycle`

### 页面控制
- `page.goto`
- `page.click`
- `page.type`
- `page.wait`
- `page.extract`
- `page.snapshot`

### 登录控制
- `session.login_parent`
- `session.login_child`
- `session.check_alive`

### 空间控制
- `workspace.open_admin`
- `workspace.invite_child`
- `workspace.verify_child_joined`

### 额度采集
- `quota.read_5h`
- `quota.read_7d`
- `usage.read_requests`
- `usage.read_tokens`
- `usage.read_messages`

---

## 当前产品建议
不要做 GUI 操作台；优先做：
- CLI 互动入口
- 后台 worker
- dashboard 只做查看与轻干预
