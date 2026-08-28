# Direct Crossterm Integration Over Ratatui

We replace `bevy_ratatui` and `ratatui` with direct `crossterm` calls and ANSI DECSTBM scroll-region escape sequences.

## Context

`bevy_repl` previously depended on `bevy_ratatui` and `ratatui` to manage raw mode and render the prompt bar. However, `bevy_ratatui` upstream releases lagged behind modern Bevy versions, forcing git fork dependencies that blocked publishing `bevy_repl` to `crates.io`. Furthermore, `bevy_repl` only requires raw-mode key polling, cursor positioning, and a sticky prompt line, making Ratatui's full widget and layout engine unnecessary overhead.

## Decision

Remove `bevy_ratatui` and `ratatui` entirely. Use `crossterm` directly for raw mode lifecycle and ANSI escape sequences (`\x1B[1;{H-1}r`) to partition the terminal viewport for sticky prompt rendering.

## Consequences

- Unblocks publishing `bevy_repl` to `crates.io` with stable, published dependencies.
- Eliminates dozens of transitive dependencies and speeds up compilation.
- Direct control over panic hooks, `Drop` restoration, and cursor positioning without fighting an external TUI framework.
- Advanced multi-widget TUI layouts are out of scope; the display is tailored specifically for an in-terminal REPL prompt with scrolling logs.
