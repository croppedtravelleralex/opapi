# PROJECTS.md

统一项目表。放在 workspace 根目录，供所有接入的 agent 读取。

## 使用说明

- 一行代表一个项目
- `状态` 建议使用：规划中 / 开发中 / 阻塞 / 已完成 / 已暂停
- `优先级` 建议使用：P0 / P1 / P2 / P3
- `最后更新` 使用 `YYYY-MM-DD`
- `路径` 尽量填写 workspace 内相对路径
- `备注` 用来写当前进度、关键风险、下一步动作

## 项目表

| 项目名 | 状态 | 优先级 | 路径 | 负责人 | 最后更新 | 备注 |
|---|---|---|---|---|---|---|
| lightpanda-automation | 规划中 | P1 | lightpanda-automation/ | 待定 | 2026-03-25 | 高性能浏览器自动化系统；规划技术栈为 Rust + SQLite + REST API + 内存任务队列 + fake runner，后续接入 lightpanda-io/browser |

## 约定

- 新项目先加到这里，再开始落目录
- 项目状态变化时，顺手更新 `最后更新`
- 如果项目有详细说明，建议在对应项目目录下补充 `README.md`
- 如果项目需要长期上下文，可在根目录或项目目录中补充 `STATUS.md` / `TODO.md`
