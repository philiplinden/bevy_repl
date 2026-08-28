# Prune and Feature-Gate Dependencies

Type: task
Status: resolved

## Question

Which dependencies should be removed, trimmed, or feature-gated to ensure `bevy_repl` is lean, fast to compile, and has zero unnecessary crates?

Specifically:
1. Remove `bevy_ratatui`, `ratatui`, `anyhow`, and `color-eyre`.
2. Add `crossterm` as the single core terminal dependency.
3. Feature-gate `tracing-subscriber` behind an optional `log` feature flag (or leverage Bevy's re-exports) so logging integration is strictly opt-in.
4. Keep `bevy_repl_derive` optional under `features = ["derive"]`.

## Answer

### 1. Pruned Core Dependencies
- **Removed**: `bevy_ratatui`, `ratatui`, `anyhow`, `color-eyre`.
- **Retained Core**:
  - `bevy`: `0.19` (default-features = false)
  - `crossterm`: `0.28` (terminal I/O & raw mode)
  - `clap`: `4.6` (CLI parsing)
  - `shell-words`: `1.1` (tokenization)
  - `ctrlc`: `3` (SIGINT safety net)

### 2. Feature Gating
- `derive`: `["bevy_repl_derive", "clap/derive"]` (default on)
- `log`: `["dep:tracing-subscriber"]` (optional tracing subscriber layer)
- `default_commands`: `["quit", "help", "clear"]`

### 3. Fast Local Builds
Configured `.cargo/config.toml` for Linux x86_64 utilizing `clang` + `lld` linker alongside nightly `-Zthreads=0` and `-Zshare-generics=y`, enabling sub-second incremental builds without requiring `dynamic_linking` in production releases.
