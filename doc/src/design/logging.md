# Routing Bevy logs to the REPL

By default, Bevy REPL integrates with Bevy's `LogPlugin` without additional
setup. To only print the REPL to stdout, disable Bevy's `LogPlugin`.

## Using an alternate TUI screen (experimental)

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
