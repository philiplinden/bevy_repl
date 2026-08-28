# Map: Native Crossterm REPL Integration

## Destination

A complete technical architecture and implementation specification for removing `bevy_ratatui` and implementing a native `crossterm` execution loop directly within Bevy 0.19's schedule sets, covering sticky prompt rendering, panic safety, custom tracing log routing, and observer-based command dispatch.

## Notes

- **Domain**: Bevy ECS, crossterm terminal I/O, ANSI DECSTBM escape sequences, tracing-subscriber layers.
- **Relevant Skills**: `domain-modeling`, `codebase-design`.
- **Working Agreement**: User writes the implementation code; this map charts and resolves the architectural specifications, data structures, system scheduling, and edge cases.
- **Display Strategy Priority**:
  1. *Primary Target*: Sticky Prompt via DECSTBM scroll region (logs scroll above pinned prompt on main screen).
  2. *Fallback Target*: Full Alternate-Screen TUI if DECSTBM adds overwhelming cross-terminal edge-case complexity.

## Decisions so far

- [[01] Terminal Lifecycle and Panic Recovery](issues/01-terminal-lifecycle-and-panic-recovery.md): Single unified `Repl` resource managing session state and direct terminal lifecycle helpers (`init_terminal`, `restore_terminal`, `clear_terminal`), eliminating `ReplTerminal` and `ReplPrompt` redundancies, with panic and SIGINT safety hooks.
- [[02] Crossterm Event Ingestion and Input Suppression](issues/02-crossterm-event-ingestion-and-input-suppression.md): Non-blocking `poll(Duration::ZERO)` in `ReplSet::Capture` with `Press | Repeat` filtering, modular `PromptKeymap`, Shift+Enter multiline, state-based input suppression, and toggle-key filtering.
- [[03] Sticky Prompt and DECSTBM Scroll Region](issues/03-sticky-prompt-and-decstbm-scroll-region.md): ANSI DECSTBM escape codes for row partitioning, change-detection prompt rendering in `ReplSet::Render`, and dynamic resize handling.
- [[05] Clap Command Parser and Observer Dispatch](issues/05-clap-command-parser-and-observer-dispatch.md): Shell-words tokenization, explicit required `to_event` mapping (eliminating silent argument loss and artificial `Default` bound), `bevy_repl_derive` support, and `Commands::trigger` observer dispatch.
- [[06] Derive Macro Ergonomics and Complexity](issues/06-derive-macro-ergonomics-and-complexity.md): Dual macro design (`#[repl_command]` attribute macro to automatically inject `clap::Parser`/`Event`/`Clone` derives alongside standard `#[derive(ReplCommand)]`), zero `Default` bound requirement, and feature gating.
- [[04] Tracing Subscriber Integration](issues/04-tracing-subscriber-integration.md): `repl_tracing_layer()` integration for Bevy's `LogPlugin`, scroll-region log positioning at row $H-1$, and automatic cursor restoration back to the prompt line.
- [[07] Prune and Feature-Gate Dependencies](issues/07-prune-and-feature-gate-dependencies.md): Removed `bevy_ratatui`, `ratatui`, `anyhow`, `color-eyre`; configured lean core dependencies and fast linker profile in `.cargo/config.toml`.

## Not yet specified

- Multi-line / word-wrapped prompt reserving $N$ bottom rows for long commands.
- Status bar / gutter line (top or bottom) for FPS, entity counts, or custom debug diagnostics.
- Command history buffer and arrow-key navigation (`Up`/`Down` history stack).
- Key modifier chords (Ctrl, Alt, Shift) comprehensive mapping table.

## Out of scope

- Full Alternate-Screen TUI mode (scope is strictly the main screen sticky prompt).
- External background runner thread (bound to in-schedule execution per ADR-0001).
- Complex multi-widget rendering pipelines (no Ratatui replacement widgets).
