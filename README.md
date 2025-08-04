# bevy_repl

An interactive REPL for headless Bevy apps powered by `clap` for command parsing
and `bevy_crossterm` for terminal input and output. The plugin creates a virtual
terminal interface within the game window that emulates an interactive shell. It
provides:

- A terminal emulation layer with input at the bottom and scrollable logs above
- Command parsing and execution using the Bevy ECS
- Integration with Bevy's logging system for unified output display
- Observer-based execution system for full control of when and where actions
  take place, and full ECS access for both read and write operations

The REPL is designed as an alternative to [makspll/bevy-console] for Bevy apps
that want a terminal-like interface without rendering or UI dependencies.

[makspll/bevy-console]: https://github.com/makspll/bevy-console

> **Warning**: This is my first public Bevy plugin, and I vibe-coded a large part
> of it. You have been warned.

## Built-in Commands

The plugin adds the following commands to the REPL by default.

| Command | Description |
| --- | --- |
| `quit`, `q`, CTRL+C | Gracefully terminate the application |
| `help` | Show clap help text |

## Design

### Headless mode

["Headless" mode] is when a Bevy app runs in the terminal without a renderer. To
run Bevy in headless mode, disable all windowing features for Bevy in
`Cargo.toml`. Then configure the schedule runner to loop forever instead of
exiting the app after one frame. Running the app from the terminal only displays
log messages from the engine to the terminal and cannot accept input.

Normally the open window keeps the app running, and the exit event happens when
closing the window. In headless mode there isn't a window to close, so the app
runs until we kill the process or another system triggers the `AppExit` event
with a keycode event reader (like press Q to quit).

["Headless" mode]:
    https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs

```toml
[dependencies]
bevy = { version = "*", default-features = false }
# replace "*" with the most recent version of bevy
```

```rust
fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Exit with Ctrl+C
    app.run();
}
```

### Console interaction

`bevy_repl` takes the idea of a Half-Life 2 debug console and brings it to
headless mode, so an app can retain command style interaction without depending
on windowing, rendering, or UI features. We accomplsh this with a trick: use
`bevy_crossterm` to create a full text user interface (TUI) that looks just like
the app running normally in headless mode, but with an area at the bottom that
supports keyboard input. Technically the app is running with a TUI, not truly
headless, but TUIs don't need windowing or a renderer so we still accomplish our
goal.

```text
┌───your terminal──────────────────────────────────────────────────────────────┐
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL                     │
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Type 'help' for commands          │
│                                                                              │
│ [Game logs and command output appear here...]                                │
│                                                                              │
│ > spawn-player Bob                                                           │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Command parsing

Use `clap` to parse commands from the REPL's input area. Commands are registered
as triggers that fire one-shot systems, and pass along the arguments and options
as context.

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Add command to Repl
    app.add_plugins(ReplPlugin)
        .repl::<QuitCommand>(on_quit);

    app.run();
}   

/// This function runs only once for each time the command event is triggered
fn on_quit(trigger: Trigger<QuitCommand>, events: EventWriter<AppExit>) {
    if trigger.verbose {
        info!("Quitting...");
    };
    events.write(AppExit::Success);
}

/// A clap parser that interprets text inputs as commands with arguments
#[derive(Parser, ReplCommand)]
#[command(name = "quit")]
struct QuitCommand {
    #[arg(short, long)]
    verbose: bool,
}
```

### Scheduling

The REPL input system set runs `First` every frame. When commands are parsed,
they trigger events that are captured in later stages of the schedule.

Command execution scheduling is handled by placing the command observers in the
schedule as needed (in `Update`, `FixedUpdate`, etc.).

There is no output or display stage. Since the REPL TUI captures all Bevy logs,
use the regular `info!` or `debug!` macros and the `RUST_LOG` environment
variable to configure messages printed to the console.

## Future Features

We plan to add the following features in future releases:

- **Error handling examples** - Show how observers handle invalid commands and parsing failures
- **Multiple observers** - Demonstrate how different systems can observe the same command

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
