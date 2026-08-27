# Map: Native Crossterm REPL Integration

## Destination

A complete technical architecture and implementation specification for removing `bevy_ratatui` and implementing a native `crossterm` execution loop directly within Bevy 0.19's schedule sets, covering sticky prompt rendering, panic safety, custom tracing log routing, and observer-based command dispatch.

## Notes

- **Domain**: Bevy ECS, crossterm terminal I/O, ANSI DECSTBM escape sequences, tracing-subscriber layers.
- **Relevant Skills**: `domain-modeling`, `codebase-design`.
- **Working Agreement**: User writes the implementation code; this map charts and resolves the architectural specifications, data structures, system scheduling, and edge cases.

## Decisions so far

- [[01] Terminal Lifecycle and Panic Recovery](issues/01-terminal-lifecycle-and-panic-recovery.md): RAII `ReplTerminal` resource with `Drop` restoration, lightweight `std::panic` hook wrapper, and dynamic raw-mode toggle on REPL enable/disable.

## Not yet specified

- Terminal resize synchronization (`SIGWINCH` / `crossterm::event::Event::Resize`) and dynamic DECSTBM region recalculation.
- Command history buffer and arrow-key navigation (`Up`/`Down` history stack).
- Key modifier chords (Ctrl, Alt, Shift) comprehensive mapping table.

## Out of scope

- Full Alternate-Screen TUI mode (scope is strictly the main screen sticky prompt).
- External background runner thread (bound to in-schedule execution per ADR-0001).
- Complex multi-widget rendering pipelines (no Ratatui replacement widgets).
