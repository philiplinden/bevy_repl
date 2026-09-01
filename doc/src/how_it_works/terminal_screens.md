# Terminal Screens

Ratatui TUIs often use an alternate screen (separate from stdout). Bevy REPL favors a "partial-TUI" that renders the prompt while keeping stdout usable.

- When REPL is active, the terminal runs in raw mode and prints to stdout.
- Prefer `bevy_repl::repl_println!` over `println!` while REPL is active to avoid cursor/newline glitches.
- If you enable a full alternate screen via `bevy_ratatui::RatatuiPlugins`, REPL still works but output behavior changes.

`repl_println!` ensures safe, consistent output:

```rust
fn on_ping(_t: Trigger<Ping>) {
    bevy_repl::repl_println!("Pong");
}

fn instructions() {
    bevy_repl::repl_println!();
    bevy_repl::repl_println!("Welcome to the Bevy REPL!");
}
```

If you are using an alternate TUI screen (like with `RatatuiPlugins`), Bevy log
messages will not be visible in the REPL unless you disable Bevy's `LogPlugin`.

If the Ratatui context is enabled (e.g.,
`bevy_ratatui::RatatuiPlugins::default()` or
`bevy_ratatui::context::ContextPlugin` is added to the app), the REPL handles
log routing like so:

- A custom `tracing` Layer captures log events and forwards them through an
  `mpsc` channel to a Non-Send resource.
- A system transfers messages from the channel into an `Event<LogEvent>`.
- You can then read `Event<LogEvent>` yourself, or use the provided system that
  prints via `repl_println!` so lines render above the prompt.

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<bevy::log::LogPlugin>(),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
        ))
        .run();
}
```
