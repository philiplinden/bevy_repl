# Vanilla Bevy from the Terminal

How a Bevy app behaves in the terminal without BevyREPL: running headless, and
where logs go.

## Headless mode

"Headless" mode is when a Bevy app runs in the terminal without a window. All
systems run as normal, such as asset loading. The app runs headless if the
`bevy_window` feature is disabled or the `WindowPlugin` is disabled.

A bare `App` runs its schedule once and exits. `MinimalPlugins` and
`DefaultPlugins` without `bevy_window` both add a `ScheduleRunnerPlugin`, which
keeps the app running in a loop until it exits.

**Bevy headless examples:**

- [examples/app/headless.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs)
- [examples/app/headless_renderer.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs)
- [examples/app/externally_driven_headless_renderer.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/externally_driven_headless_renderer.rs)

> [!NOTE]
> Bevy gets keyboard events from the window, so a headless app sees no
> keystrokes, and `MinimalPlugins` doesn't add `InputPlugin` at all. The REPL
> reads keys straight from the terminal with `crossterm` instead of through
> Bevy's input system, so typing at the prompt doesn't depend on either.

## Logging

Bevy's `LogPlugin` (enabled by feature flag `bevy_log`) sets up the engine to
print log messages to the terminal, with or without a renderer or app window. In
a headless app, this is the primary way information and state is conveyed.

The `bevy_log` feature is enabled by default and the `LogPlugin` is included in
Bevy's `DefaultPlugins` group. Additional instrumentation and integration with
[`tracing` and `tracing-subscriber`](https://github.com/tokio-rs/tracing)
crates via the `trace` feature, though it is completely optional.

While the `LogPlugin` is _not_ a member of the `MinimalPlugins` group, it
is compatible with headless mode and has relatively small overhead in terms of
compiling and running the Bevy app. Custom logging behavior is set up by modifying
the `LogPlugin` configuration when building `DefaultPlugins` or by disabling it
altogether and replacing it with a custom logging implementation. BevyREPL does
neither. See [Preserving normal Logging](../dev/logging.md).

**Bevy logging examples:**

- [examples/app/logs.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/logs.rs)
- [examples/app/log_layers.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/log_layers.rs)
- [examples/app/log_layers_ecs.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/log_layers_ecs.rs)

## Minimal Example

A headless app that runs at 60 fps, logs, and exits on Ctrl+C.

```toml
[dependencies]
bevy = { version = "0.20", default-features = false, features = ["bevy_log"] }
```

```rust,ignore
use std::time::Duration;
use bevy::{
    app::{ScheduleRunnerPlugin, TerminalCtrlCHandlerPlugin},
    log::LogPlugin,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            // MinimalPlugins already has a ScheduleRunnerPlugin.
            // Replace it with one that waits between frames, so
            // the loop doesn't spin a core at 100%.
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )),
            LogPlugin::default(),
            // Exit on Ctrl+C (DefaultPlugins has this, MinimalPlugins doesn't)
            TerminalCtrlCHandlerPlugin,
        ))
        .run();
}
```
