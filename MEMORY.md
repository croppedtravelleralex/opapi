# MEMORY.md

- 用户的 GitHub 仓库之一：`croppedtravelleralex/croppedtravelleralex-auto-open-browser`。
- 本机对应项目目录：`/root/projects/lightpanda-automation`。
- 用户正在推进 `lightpanda-automation` 项目，定位为偏个人学习与研发使用的高性能浏览器自动化系统。
- `lightpanda-automation` 的核心技术路线：`Rust + SQLite + REST API + 内存任务队列 + fake runner`，后续接入 `lightpanda-io/browser` 作为真实浏览器执行引擎。
- 用户认可该项目应先定义最终效果、最终功能、工程结构和推进顺序，再推进代码实现，避免项目在自动推进过程中迷失方向。
- `lightpanda-automation` 的长期关键方向包括：高级浏览器指纹能力、代理池、所有访问强制走代理池、代理地区匹配目标地区、可用代理比例维持在 `40%-60%` 并随并发动态调整、代理池自生长、持续抓取代理工具（优先基于开源项目改造）。
- 已成功为 OpenClaw 接入 Telegram：bot 为 `@ChuancroppedBot`；用户 Telegram sender id 为 `6553994692`；默认交互规则为群里仅被 @ 时响应，私聊可直接使用。
- 用户要求：讨论项目推进时，每次默认给出 **4 个接下来适合实现的功能**，并按优先级排序，**最适合的排第 1**。
- 用户要求：回复中该加粗的地方要加粗，尤其是结论、优先项和关键提醒。
- 用户要求：项目推进时，**每两轮**要主动回顾项目，默认加入 **找 bug、性能评分、改进建议** 这三个动作，不要等用户每次提醒。
- 已解决一次 OpenClaw 无响应问题：根因是网关 `bind` 配置为 loopback，仅监听本地回环地址，导致外部网络无法访问；在 tailscale 为 `serve` 模式下无法直接改为 lan 绑定。通过重启 OpenClaw 服务应用当前配置后恢复正常；后续遇到同类问题应优先检查 gateway bind 与外部可达性。
