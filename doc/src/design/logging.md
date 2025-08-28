# Routing Bevy logs to the REPL

You can optionally route logs produced by Bevy's `tracing` pipeline to the REPL
so they are formatted in the REPL's renderer. Otherwise, `bevy::log::LogPlugin`
will print logs directly to stdout. This means that if you are using an
alternate TUI screen (like with the default `RatatuiPlugins`), Bevy log messages
will not be visible in the REPL unless you disable Bevy's `LogPlugin`.

When the default `LogPlugin` is disabled, the REPL handles log routing like so:

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
            // 2) Disable Bevy's stdout logger to prevent duplicate/garbled output
            DefaultPlugins.build().disable::<bevy::log::LogPlugin>(),
            ReplPlugins,
        ))
        .run();
}
```
