# Bevy from the Terminal

Bevy's input system doesn't make it easy to interact with an app with a Command
Line Interface (CLI) or command console. Out of the box, text input is handled
by a user interface and parsing the text into events or other game behavior is
left to the app developer.

1. Text input requires a windowed app with a renderer, and the text is handled
   by a GUI element, like
   [bevy-console](https://github.com/makspll/bevy-console); or
2. The default renderer is replaced by a TUI (which is just a renderer that
   happens to not leave the terminal), like
   [bevy_ratatui](https://github.com/ratatui/bevy_ratatui) and
   [bevyterm](https://github.com/Mimea005/bevyterm).
3. There is no windowing system or renderer, but then consequently no text
   input system.

## Headless mode

"Headless" mode is when a Bevy app runs in the terminal without a window. All
systems run as normal, such as input detection and asset loading, but the app
exits after one loop iteration unless it is configured to run indefinitely. The
app runs headless if the `bevy_window` feature is disabled or the `WindowPlugin`
is disabled.

**Bevy headless examples:**
- [examples/app/headless.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs)
- [examples/app/headless_renderer.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs)
- [examples/app/externally_driven_headless_renderer.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/externally_driven_headless_renderer.rs)

> [!WARNING]
> The `keyboard` feature is required for the engine to recognize keystrokes as
> input events. It is usually disabled in headless mode by default. Managing keyboard
> inputs is kind of a pain anyway, and parsing text rather than registering keypresses
> ends up being an exercise in UI frustration (even when there's no UI!). Be
> sure to also enable plugins like `InputPlugin` so the app can handle keyboard 
> inputs for critical behavior such as quitting the app.

## Logging

Bevy's `LogPlugin` (enabled by feature flag `bevy_log`) sets up the engine to
print log messages to the terminal, with or without a renderer or app window. In
a headless app, this is the primary way information and state is conveyed.

The `bevy_log` feature is enabled by default and the `LogPlugin` is included in
Bevy's `DefaultPlugins` group. Additional instrumentation and integration with
[`tracing` and `tracing-subscriber`](https://github.com/tokio-rs/tracing)
creates via the `trace` feature, though it is completely optional.

While the `LogPlugin` is _not_ a member of the `MinimalPlugins` group, it 
is compatible with headless mode and has relatively small overhead in terms of
compiling and running the Bevy app. Custom logging behavior is set up by modifying
the `LogPlugin` configuration when building `DefaultPlugins` or by disabling it
altogether and replacing it with a custom logging implementation. The
"canonical" way to set up logging is to add a custom log layer or to capture
log messages via a custom sink.

**Bevy logging examples:**
- [examples/app/logs.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/logs.rs)
- [examples/app/log_layers.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/log_layers.rs)
- [examples/app/log_layers_ecs.rs](https://github.com/bevyengine/bevy/blob/main/examples/app/log_layers_ecs.rs)

## Minimal Example

```toml
[dependencies]

bevy = { 
  version = "*", # replace "*" with the most recent version of bevy
  default-features = false, # disable all default features for demonstration
  features = ["bevy_log" ] # see: https://docs.rs/bevy/latest/bevy/#features
}
```

```rust
//! A minimal headless app that runs at 60 fps and exits on Ctrl+C. This example
//! has the same behavior whether or not Bevy's default features are enabled.
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        // Only what's necessary to run Bevy
        MinimalPlugins,
        // Exit with Ctrl+C (included in DefaultPlugins)
        TerminalCtrlCHandlerPlugin,
        // The ScheduleRunnerPlugin handles the app run loop. In a headless Bevy
        // app (no window) using the schedule runner with no frame wait
        // configured, the loop runs as fast as possible (busy-loop on native),
        // consuming a core. Run at 60 fps so it doesn't melt your CPU.
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    app.run();
}
```
