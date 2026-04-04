# progress.md

## 2026-04-04 Session
- Initialized planning files for the new discovery-driven registration automation feature.
- Next: inspect current codex/mailbox route and schema files, then implement the highest-value minimal path.

- Implemented discovery-driven automation target flow with sqlite-backed targets/attempt history, try-and-mark path, success recording, failure optimization hints, and tests.
- Validation: cargo test -q => 17 + 17 + 30 + 0 green.

- Upgraded mailbox pool toward scalable quality model: quality_score, expansion_tier, reservation_count, capacity events, overview/expand endpoints, and tests.

- Added fixed mailbox auto-tiering rules: promote/demote/freeze run endpoint, poll-time score-tier transitions, and regression tests.
