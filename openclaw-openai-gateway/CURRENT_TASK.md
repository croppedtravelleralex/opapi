# 当前任务（openclaw-openai-gateway）

当前任务：**把项目进度口径纠正为“双进度制”，并把控制平面继续推进到 SQLite 主读源阶段。**

本轮目标：
1. 补 `FINAL_FEATURE_MAP.md`
2. 把 `STATUS.md` 改成“双进度口径”
3. 把 `CURRENT_DIRECTION.md` 改成“数据库驱动的最小控制平面 skeleton”
4. 下一轮继续推进：
   - 把 audit 持久化到 SQLite
   - 让 `provider_capabilities / model_availability` 真正参与 routing
   - 继续扩 AccountPool / Governance / ChangePlan

本轮不追求：
- 一口气做完整多 Provider 行为
- 一口气做完整 account pool / automation
- 一口气做完整治理面
