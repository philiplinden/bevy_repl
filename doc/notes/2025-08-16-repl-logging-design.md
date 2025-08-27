# bevy_repl: Logging + Prompt Rendering Rework (2025-08-16)

## Summary
We refined how logging integrates with REPL prompt rendering, separating minimal (in-frame logs) from pretty/alternate-screen (scroll region) behavior. We also clarified configuration patterns and examples using `LogCaptureConfig`, `InFrameLogPlugin`, and Bevy's `LogPlugin.custom_layer`.

## Key design decisions
- **Mode-specific logging behavior**
  - Minimal (aka InFrame) mode renders logs inside the ratatui frame using a `LogBuffer`.
  - Pretty (alternate-screen) mode uses a terminal scroll region above a fixed prompt; no in-frame log rendering by default.
- **Plugin wiring by prompt mode**
  - `PromptPlugin::minimal()` sets `PromptMode::InFrame` and now adds `InFrameLogPlugin` automatically.
  - `PromptPlugin::pretty()` sets `PromptMode::AlternateScreen` and does not add `InFrameLogPlugin`.
- **Simplified plugin groups**
  - `ReplPlugins` no longer embeds in-frame logging. The prompt plugin governs logging behavior.
- **Two supported logging approaches**
  - A) REPL-orchestrated capture (`InFrameLogPlugin` + `LogCaptureConfig { init_subscriber: true }`).
  - B) Bevy `LogPlugin.custom_layer` with `CapturePlumbingPlugin` + `custom_layer(app)` and `init_subscriber: false`.
- **Clear trade-offs and duplication notes**
  - In-frame `LogBuffer` lines are minimal (`level + message`).
  - Bevy fmt output continues to stdout when using `LogPlugin`, which can duplicate logs with in-frame rendering.
  - For formatted logs inside REPL (no stdout), prefer `tracing_to_repl_fmt()` and disable Bevy's `LogPlugin`.

## What changed vs before
- **Before**
  - `ReplPlugins` sometimes attempted to carry logging setup implicitly.
  - `InFrameLogPlugin` could be added via groups, making pretty mode behavior ambiguous.
  - Minimal vs pretty logging behavior was less explicit; examples were limited to the custom-layer path.
- **After**
  - `ReplPlugins` is lean; no implicit in-frame logging.
  - `PromptMode` introduced in `src/prompt/mod.rs` and bound to the prompt presets:
    - `PromptMode::InFrame` for minimal.
    - `PromptMode::AlternateScreen` for pretty.
  - `PromptPlugin::build()` adds `InFrameLogPlugin` only when mode is `InFrame`.
  - README now explains both approaches, trade-offs, and how to avoid duplication.
  - New example `examples/log_inframe.rs` shows REPL-orchestrated capture.

## Affected files (high-level)
- `src/prompt/mod.rs`
  - Added `PromptMode` and mode-aware wiring of `InFrameLogPlugin` in `PromptPlugin::build()`.
  - `PromptPlugin::minimal()` sets mode to `InFrame`; `pretty()` sets `AlternateScreen`.
- `src/prompt/renderer/minimal.rs`
  - Minimal renderer continues to draw recent `LogBuffer` lines above the prompt within the frame.
- `src/plugin.rs`
  - `ReplPlugins` and `MinimalReplPlugins` no longer include `InFrameLogPlugin`; prompt mode controls it.
- `src/log_ecs.rs`
  - `InFrameLogPlugin` orchestrates capture → ECS → `LogBuffer` and respects `LogCaptureConfig`.
- `README.md`
  - Clarified `LogCaptureConfig` fields; documented Approach A and B with notes on duplication and formatting.
- `examples/`
  - `log_inframe.rs`: REPL-orchestrated capture (turnkey, minimal formatting in-frame).
  - `log_custom_layer.rs`: Bevy `LogPlugin.custom_layer` integration.
  - `pretty_custom.rs`: updated to set `PromptMode::AlternateScreen` when customizing pretty.

## API surface (current)
- `log_ecs::InFrameLogPlugin` — enables capture + in-frame `LogBuffer` rendering pipeline.
- `log_ecs::LogCaptureConfig { level, capacity, init_subscriber }` — configures capture and buffer.
- `log_ecs::CapturePlumbingPlugin` + `custom_layer(app)` — Bevy `LogPlugin` integration path.
- `prompt::PromptPlugin::{minimal, pretty}` — selects renderer and mode.
- `prompt::PromptMode::{InFrame, AlternateScreen}` — drives logging integration behavior.

## Migration guidance
- If you want minimal/in-frame logs:
  - Use `ReplPlugins` (which includes `PromptPlugin::minimal()`), insert `LogCaptureConfig`, and you’re done. No need to add `InFrameLogPlugin` manually.
- If you use pretty/alternate-screen:
  - Do not enable `InFrameLogPlugin`. Keep `LogPlugin` or use `tracing_to_repl_fmt()` to print above the prompt.
- If you need Bevy formatting but also want REPL capture:
  - Use `CapturePlumbingPlugin` + `LogPlugin.custom_layer(custom_layer)` and `LogCaptureConfig { init_subscriber: false }`. Expect stdout + in-frame duplication unless you disable stdout fmt.
- If you want formatted logs inside REPL only:
  - Use `tracing_to_repl_fmt()` and disable `LogPlugin` to avoid duplicates.

## Open items / next steps
- Pretty-mode: optional in-frame rendering style (if ever desired) behind a toggle.
- Optional builder-style APIs for common setups (e.g., `ReplPlugins::with_inframe_logs()` convenience).
- More examples/tests for duplication scenarios and fmt-to-REPL path.
