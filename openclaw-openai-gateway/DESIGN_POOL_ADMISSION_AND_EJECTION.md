# DESIGN_POOL_ADMISSION_AND_EJECTION.md

## 目标
建立额度池的准入、限流、冷却、出池规则。

---

## 入池原则
只有子号满足以下条件，才可进入额度池：
1. 已加入空间
2. 已验证空间
3. 网页额度读取正常
4. 子号未封
5. 空间未封
6. 风险状态正常
7. 短时间请求频率未超阈值
8. 最近错误率未超阈值

---

## 评分分级

### Green
- 5h 额度健康
- 7d 额度健康
- 频率正常
- 风险正常

动作：
- 正常承接请求

### Yellow
- 5h 或 7d 额度下降
- 或短时间频率偏高

动作：
- 低权重承接
- 限流

### Red
- 额度低于 5%
- 子号异常
- 空间异常
- 登录态异常
- 风险异常

动作：
- 立即出池

---

## 立即出池条件
- 子号被封
- 空间被封
- 额度低于 5%
- 短时间请求过于频繁
- 达到请求数量阈值
- 登录态失效
- 网页额度连续采集失败
- 风控信号异常

---

## 出池后的状态
- `cooling`：稍后重试
- `suspended`：等待人工确认
- `dead`：彻底弃用

---

## 池成员模型
### pool_members
- `id`
- `child_account_id`
- `pool_status`
- `admission_level`
- `weight`
- `current_load`
- `cooldown_until`
- `last_success_at`
- `last_failure_at`
