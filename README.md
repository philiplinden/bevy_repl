# bevy_repl

An interactive REPL for headless Bevy apps powered by `clap` for command parsing
and `bevy_ratatui` for terminal input and output. The plugin adds a togglable
text input area below the terminal output for interaction even in headless mode.

- Unobtrusive, toggleable TUI console below normal terminal output
- Command parsing and CLI features from `clap`  
- Observer-based command execution system
- Full Bevy ECS access for both read and write operations

The REPL is designed as an alternative to [makspll/bevy-console] for Bevy apps
that want a terminal-like console to modify the game at runtime without
implementing a full TUI or rendering features.

[makspll/bevy-console]: https://github.com/makspll/bevy-console

> This is my first public Bevy plugin, and I vibe-coded a large part
> of it. **You have been warned.**

## Features

Enable built-in commands with feature flags. Each command is enabled separately
by a feature flag. Use the `default-commands` feature to enable all built-in
commands.

| Feature Flag | Command | Description |
| --- | --- | --- |
| `default_commands` | `quit`, `help`, `clear` | Enable all built-in commands |
| `quit` | `quit`, `q`, CTRL+C | Gracefully terminate the application |
| `help` | `help` | Show clap help text |
| `clear` | `clear` | Clear the REPL input buffer |

Clap features are technically supported, but have not been tested. Override the
`clap` features in your `Cargo.toml` to enable or disable additional features.

## Default Keybinds

| Key | Action |
| --- | --- |
| ``` | Toggle REPL visibility |
| `Enter` | Submit command |
| `Esc` | Clear input buffer |
| `Left/Right` | Move cursor |
| `Home/End` | Jump to start/end of line |
| `Backspace` | Delete character before cursor |
| `Delete` | Delete character at cursor |
| `CTRL+C` | Exit application |

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

### REPL Console

`bevy_repl` takes the idea of a Half-Life 2 debug console and brings it to
headless mode, so an app can retain command style interaction without depending
on windowing, rendering, or UI features.

Instead of rendering a fullscreen text user interface (TUI), which would kinda
defeat the purpose of headless mode, we render a small "partial-TUI" at the
bottom of the terminal that supports keyboard input. The normal headless output
is shifted up to make room for the input console, and everything else is
printed to the terminal normally. Technically the app is running with a TUI,
not truly headless, but we are deliberately attempting to preserve the illusion
of headless mode.

```toml
[dependencies]
bevy_repl = { version = "0.1.0", features = ["default-commands"] }
```

**REPL disabled (regular headless mode):**

```text
┌───your terminal──────────────────────────────────────────────────────────────┐
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL                     │
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Type 'help' for commands          │
│                                                                              │
│ [Game logs and command output appear here...]                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

**REPL enabled:**

```text
┌───your terminal──────────────────────────────────────────────────────────────┐
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL                     │
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Type 'help' for commands          │
│                                                                              │
│ [Game logs and command output appear here...]                                │
│                                                                              │
┌───REPL───────────────────────────────────────────────────────────────────────┐
│ > spawn-player Bob                                                           │
└──────────────────────────────────────────────────────────────────────────────┘
```

When the REPL is disabled, keycode input events are forwarded to Bevy via
`bevy_ratatui` as normal. The REPL is toggled with a Bevy KeyCode event (``` by
default).

When the REPL is enabled, keycode forwarding to Bevy is disabled (except for the
REPL toggle key) and all key strokes are consumed by the REPL. This is to avoid
passing events to Bevy when you are typing a command. Disable the REPL to return
to normal headless mode.

### Command parsing

Input is parsed via `clap` commands and corresponding observer systems that
execute when triggered by the command.

Use clap's [builder pattern] to describe the command and its arguments or
options. Then add the command and its observer to the app with
`.add_repl_command<Command>(observer)`. The observer is a one-shot system that
receives a trigger event with the command's arguments and options.

**Note:** I haven't tested clap's derive pattern yet.

[builder pattern]: https://docs.rs/clap/latest/clap/_tutorial/index.html#tutorial-for-the-builder-api

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    let mut app = App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
        ));

    app.add_plugins((
        ReplPlugin,
        ReplDefaultCommandsPlugin,
    ))
    .add_repl_command::<SayCommand>()
    .add_observer(on_say);

    app.run();
}

struct SayCommand {
    message: String,
}

impl ReplCommand for SayCommand {
    fn command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .short('m')
                    .long("message")
                    .help("Message to say")
                    .required(true)
                    .takes_value(true)
            )
    }
}

fn on_say(trigger: Trigger<SayCommand>) {
    println!("{}", trigger.message);
}
```

### Scheduling

The REPL reads input events and emits trigger events alongside the `bevy_ratatui`
[input handling system set](https://github.com/cxreiff/bevy_ratatui/blob/main/src/crossterm_context/event.rs).
The REPL text buffer is updated and emits command triggers during
`InputSet::EmitBevy`. The prompt is updated during `InputSet::Post` to reflect
the current state of the input buffer.

All REPL input systems run in the `Update` schedule, but as they are
event-based, they may not run every frame. Commands are executed in the
`PostUpdate` schedule as observers.

For headless command output, use the regular `info!` or `debug!` macros and the
`RUST_LOG` environment variable to configure messages printed to the console or
implement your own TUI panels with `bevy_ratatui`.

## Aspirations

- [x] **Derive pattern** - Seamlessly integrate with clap's derive pattern
- [x] **Support for games with rendering and windowing** - The REPL is designed to
  work from the terminal, but the terminal normally prints logs when there is a
  window too. The REPL still works from the terminal while using the window for
  rendering if the console is enabled.
- [ ] **Command history** - Use keybindings to navigate past commands
- [ ] **Customizable keybinds** - Allow the user to configure the REPL keybinds for
  all REPL controls, not just the toggle key.
- [ ] **Help text and command completion** - Use `clap`'s help text and completion
  features to provide a better REPL experience and allow for command discovery.
- [ ] **Support for games with TUIs** - The REPL is designed to work as a sort of
  sidecar to the normal terminal output, so _in theory_ it should be compatible
  with games that use an alternate TUI screen. Who knows if it actually works.

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
