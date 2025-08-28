# Design

This section documents the design of the REPL and its components. I include it
here as a reference for myself and for anyone who wants to understand how the
REPL works.

## Headless Bevy

The REPL is designed to be an interactive console for the Bevy app at runtime.
It runs in the terminal while your Bevy app is running, even in headless mode.

"Headless" mode is when a Bevy app runs in the terminal without a window. All
systems run as normal, such as input detection and asset loading, but the app
exits after one loop iteration unless it is configured to run indefinitely. The
app runs headless if the `bevy_window` feature is disabled or the `WindowPlugin`
is disabled.

**Bevy headless examples:**
- https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs
- https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs

### Headless app with default features except windowing

The preferred way to run a Bevy app headless is to disable default bevy features
and explicitly add the desire features, leaving out `bevy_winit` and
`bevy_window`. (Note that Bevy Repl requires `bevy_log` and `trace` features.)

```toml
[dependencies]

bevy = { 
  version = "*", # replace "*" with the most recent version of bevy
  default-features = false,
  features = [
    "bevy_log", "trace", # Bevy REPL needs `bevy_log` and `trace`.
    # include all the other feature flags you need here.
    # see: https://docs.rs/bevy/latest/bevy/#features
  ]
}
```

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        // with bevy_window and bevy_winit disabled, those plugins aren't in
        // DefaultPlugins. All we have to do is tell the runner to run at 60 fps
        // so it doesn't consume the whole CPU core.
        DefaultPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Exit with Ctrl+C
    app.run();
}
```

### Minimal headless app (no default features)

Even with all the default features, Bevy ships `MinimalPlugins` with the minimum
set of plugins required to run a Bevy app. Be sure to also enable `InputPlugin`
so the app can handle keyboard inputs, like for the REPL or Ctrl+C interrupts.

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::input::InputPlugin::default(),
        // The ScheduleRunnerPlugin handles the app run loop. In a headless Bevy
        // app (no window) using the schedule runner with no frame wait
        // configured, the loop runs as fast as possible (busy-loop on native),
        // consuming a core. Run at 60 fps so it doesn't melt your CPU.
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Exit with Ctrl+C
    app.run();
}
```

### Headless app with default features and windowing disabled

If you need to keep the windowing features, you can disable the `WindowPlugin`
and `WinitPlugin` to run the app in headless mode.

**Tip:** Bevy REPL still runs in the terminal even if you spawn windows, so this
is probably only useful if you are running the app in CI or some other headless
environment.

```rust
use bevy::{
    prelude::*, // WindowPlugin is in the prelude
    window::ExitCondition,
    winit::WinitPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                // Don't make a new window at startup
                primary_window: None,
                // Don’t automatically exit due to having no windows.
                // Instead, run until an `AppExit` event is produced.
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            // WinitPlugin will panic in environments without a display server.
            .disable::<WinitPlugin>(),
        // We still want to set the FPS so the app doesn't melt the CPU.
        // ScheduleRunnerPlugin replaces the bevy_winit app runner, though, so
        // disabling the windowing plugins is redundant.
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ),
    ));

    // Exit with Ctrl+C
    app.run();
}
```
