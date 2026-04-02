# DESIGN_QUOTA_OBSERVATION.md

## 目标
通过网页端展示持续观测可反代额度与使用量，不依赖官方 API 假设。

---

## 观测对象

### 核心额度指标
- `quota_5h_percent`
- `quota_7d_percent`

### 使用量指标
- `request_count`
- `token_count`
- `message_count`

### 辅助信号
- 页面更新时间
- 抓取可信度
- 连续抓取失败次数
- 最近异常事件

---

## 数据模型建议

### quota_snapshots
- `id`
- `child_account_id`
- `observed_at`
- `quota_5h_percent`
- `quota_7d_percent`
- `request_count`
- `token_count`
- `message_count`
- `source_page`
- `confidence`
- `read_ok`
- `error_reason`

---

## 采集要求

### 采集来源
- 只认网页端展示
- 在指纹浏览器环境中执行
- 支持 DOM / 文本 / 快照多信号提取

### 采集节奏
建议分层：
- 高活跃子号：更高频
- 冷却/异常子号：降频
- 已死亡子号：停止主动采集

---

## 风险判断
以下情况应标记为风险：
- 额度页连续读取失败
- 页面结构变化导致解析异常
- 额度条异常归零
- 请求数短时间激增
- token/message 增长异常

---

## 输出用途
采集结果用于：
1. 决定能否入池
2. 决定是否立即出池
3. 决定是否限流
4. 给反代层做调度参考
5. 给治理层做审计与回放
