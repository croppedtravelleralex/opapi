# findings.md

## 2026-04-04
- Existing system already has auto-register, verification tasks, mailbox pool, mailbox poll, autoloop, safety gating, and worker endpoints.
- New user request expands scope from fixed child-email registration to discovery-driven automation suitability testing and result reporting.
- Likely integration point is under `src/routes/codex.rs` with SQLite-backed tables plus smoke tests in `tests/http_smoke.rs`.
- Current host/runtime check results:
  - Ubuntu 24.04.4 LTS, kernel 6.8.0-101-generic
  - Rust 1.94.1 / Cargo 1.94.1 / Node 22.22.2 / npm 10.9.7 / Git 2.43.0
  - Root filesystem 59G total / 19G free, memory 3.6GiB with available headroom ~1.7GiB
  - OpenClaw gateway runs on loopback and is exposed through Tailscale Serve
  - External listeners include `sshd:22` and `cupsd:631`
  - `ufw` and `nft` are not installed, so host has no obvious managed firewall layer
  - `openclaw security audit --deep` reports 2 critical and 9 warnings; highest-priority issues are flagged plugins (`adp-openclaw`, `openclaw-weixin`), permissive exec trust settings, missing trusted proxies, and shared-runtime exposure risk.
