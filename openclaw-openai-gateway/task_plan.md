# task_plan.md

## Goal
为 opapi 增加“自动查找可自动化邮箱注册”的能力：候选目标发现 → 尝试注册 → 记录是否可自动化 → 若失败则给出可优化方向 → 注册成功记录关键资料 → 注册失败向用户汇报。

## Constraints
- 继续围绕未来的指纹浏览器入口架构，但当前不依赖外部真实 API。
- 优先做内部能力闭环、状态机、记录与报告。
- 保持安全优先，避免泄露敏感凭据。
- 改完必须测试、commit、push。

## Phases
- [in_progress] Phase 1: Inspect current auto-register/mailbox code and identify the minimal integration path for target discovery + attempt/result recording.
- [pending] Phase 2: Add schema and route/service support for discovered targets, attempt history, capability marking, and failure improvement suggestions.
- [pending] Phase 3: Wire discovery/attempt flow into registration worker/autoloop and add user-facing reporting payloads.
- [pending] Phase 4: Add/adjust tests for discovery, success recording, failure reporting, and optimization hints.
- [pending] Phase 5: Validate, commit, and push.

## Errors Encountered
- None yet.
