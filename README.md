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

Use the `derive` feature to support clap's derive pattern for REPL commands.
`#[derive(ReplCommand)]` will automatically implement the `ReplCommand` trait
and create an event with the command's arguments and options. Configure the
response by adding an observer for the REPL command like normal.

Enable built-in commands with feature flags. Each command is enabled separately
by a feature flag. Use the `default_commands` feature to enable all built-in
commands.

| Feature Flag | Command | Description |
| --- | --- | --- |
| `default_commands` | `quit`, `help`, `clear` | Enable all built-in commands |
| `quit` | `quit`, `q`, CTRL+C | Gracefully terminate the application |
| `help` | `help` | Show clap help text |
| `clear` | `clear` | Clear the REPL input buffer |

Clap features are technically supported, but have not been tested. Override the
`clap` features in your `Cargo.toml` to enable or disable additional features.

## Known Issues

- Input area does not stay pinned to the bottom of the terminal, and the prompt symbol does not display.
- Runtime toggle is not supported in v1 (planned for a future version).
- Key events may leak through the REPL to Bevy when the REPL is enabled.
- Built-in `help` and `clear` commands are currently non-functional.

## Usage

The REPL is designed to be used in headless mode, but it can be used in windowed
mode too through the terminal while the app is running.

### REPL lifecycle (v1)

For v1 there is no runtime toggle. The REPL is either enabled at startup or inert for the entire run.

- __Configure at startup__: Use `ReplPlugin::enabled()`, `ReplPlugin::disabled()`, or `ReplPlugin::with_enabled(bool)`.
- __When enabled__: The plugin initializes terminal raw mode and inserts a `ReplContext` at startup. REPL systems run and consume input intended for the prompt.
- __When disabled__: No REPL resources or systems are registered; the application behaves as if the REPL is absent.
- __Shutdown__: On `AppExit`, raw mode is restored and `ReplContext` is removed.

Internally, this lifecycle is handled with event-based observers (Enable on startup; Disable on exit). There is intentionally no key-based toggle in v1.

Trigger commands by typing them in the REPL input buffer and pressing `Enter`.
The REPL will parse the command and trigger an event with the command's arguments
and options.

### Builder pattern (default)

1. Make a Bevy event struct that represents the command and its arguments and
   options. This is the event that will be triggered when the command is executed.
2. Implement the `ReplCommand` trait for the event struct.
   1. `fn clap_command() -> clap::Command` - Use the `clap` builder pattern to
      describe the command and its arguments or options.
   2. `fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self>` - Implement
      the `to_event` method to convert the command's arguments and options into
      the event struct. This is where you validate the command's arguments
      and options and map them to the event fields or return an error if they are
      invalid. If the command has no arguments or options, return `Ok(Self)`.
      **Tip:** If the command has no arguments or options, implement the `Default`
      trait. You don't implement `to_event` in this case, since the default
      implementation will return `Ok(Self)`.
3. Add the command to the app with `.add_repl_command<YourReplCommand>()`.
4. Add an observer for the command with `.add_observer(your_observer)`. The
   observer is a one-shot system that receives a trigger event with the command's
   arguments and options.

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct SimpleCommandWithoutArgs;

impl ReplCommand for SimpleCommandWithoutArgs {
    fn clap_command() -> clap::Command {
        clap::Command::new("simple")
            .about("A simple command")
    }
}

fn on_simple(_trigger: Trigger<SimpleCommandWithoutArgs>) {
    println!("You triggered a simple command without args");
}

struct CommandWithArgs {
    arg1: String,
    arg2: String,
}

impl ReplCommand for CommandWithArgs {

    fn clap_command() -> clap::Command {
        clap::Command::new("command")
            .about("A command with args")
            .arg(clap::Arg::new("arg1").required(true))
            .arg(clap::Arg::new("arg2").required(true))
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(CommandWithArgs {
            arg1: matches.get_one::<String>("arg1").unwrap().clone(),
            arg2: matches.get_one::<String>("arg2").unwrap().clone(),
        })
    }
}

fn on_command_with_args(trigger: Trigger<CommandWithArgs>) {
    println!("You triggered a command with args: {} {}", trigger.arg1, trigger.arg2);
}

fn main() {
    App::new()
        .add_plugins((
            // Run headless in the terminal
            MinimalPlugins.set(
                bevy::app::ScheduleRunnerPlugin::run_loop(
                    Duration::from_secs_f32(1. / 60.)
                )
            )),
            ReplPlugin,
        ))
        .add_repl_command::<SimpleCommandWithoutArgs>()
        .add_observer(on_simple)
        .add_repl_command::<CommandWithArgs>()
        .add_observer(on_command_with_args);
}
```

### Derive pattern (requires `derive` feature)

Enable the `derive` feature in your `Cargo.toml` to use the derive pattern.

```toml
[dependencies]
bevy_repl = { version = "0.1.0", features = ["derive"] }
```

Then derive the `ReplCommand` trait on your command struct along with clap's
`Parser` trait. Add the command to the app with `.add_repl_command<YourReplCommand>()`
and add an observer for the command with `.add_observer(your_observer)` as usual.

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

#[derive(ReplCommand, Parser, Default, Event)]
struct SimpleCommandWithoutArgs;

#[derive(ReplCommand, Parser, Event, Default)]
#[clap(about = "A command with args")]
struct CommandWithArgs {
    #[clap(short, long)]
    arg1: String,
    #[clap(short, long)]
    arg2: String,
}
```

### Default Keybinds

When the REPL is enabled, the following keybinds are available:

| Key | Action |
| --- | --- |
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
printed to the terminal normally. The app is truly running headless, and the
"partial-TUI" is directly modifying the terminal output with `crossterm`.

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

### Command parsing

Input is parsed via `clap` commands and corresponding observer systems that
execute when triggered by the command.

Use clap's [builder pattern] to describe the command and its arguments or
options. Then add the command to the app with
`.add_repl_command<YourReplCommand>()`. The REPL fires an event (e.g.
`YourReplCommand`) when the command is parsed from the prompt.

Make an observer for the command with `.add_observer(your_observer)`. The
observer is a one-shot system that receives a trigger event with the command's
arguments and options. As a system, it is executed in the `PostUpdate` schedule
and has full access to the Bevy ECS.

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

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(SayCommand {
            message: matches.get_all::<String>("message").unwrap().join(" "),
        })
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

### Directly modifying the terminal (to-do)

The REPL uses `crossterm` events generated by `bevy_ratatui` to read input events
from the keyboard. When the REPL is enabled, the terminal is in raw mode and the
REPL has direct access to the terminal cursor. The crate uses observers to
disable raw mode when the REPL is disabled or the app exits. If raw mode isn't
handled correctly, the terminal cursor may be left in an unexpected state.

Keycode forwarding from crossterm to Bevy is disabled (except for the
REPL toggle key) to avoid passing events to Bevy when you are typing a command.
Disabling the REPL returns the terminal to normal headless mode, and keycodes
are propagated to Bevy as normal.

We use Bevy keycode events for toggle behavior so that the REPL can be toggled
when the terminal is NOT in raw mode. This is to avoid the need to place the
terminal in raw mode even when the REPL is disabled. This is a tradeoff
between simplicity and utility. It would be simpler to enable raw mode all the
time and detect raw keycode commands for the toggle key, then forward the raw
inputs to Bevy as normal keycode events. However, this means that the app input
handling fundamentally changes, even when the REPL is disabled. For development,
it is more useful to have the app behave exactly as a normal headless app when
the REPL is disabled to preserve consistency in input handling behavior.

## Aspirations
- [x] **Derive pattern** - Describe commands with clap's derive pattern.
- [ ] **Toggleable** - The REPL is disabled by default and can be toggled. When
  disabled, the app runs normally in the terminal, no REPL systems run, and the
  prompt is hidden.
- [ ] **Pretty prompt** - Show the prompt in the terminal below the normal
  stdout, including the current buffer content.
- [x] **Support for games with rendering and windowing** - The REPL is designed to
  work from the terminal, but the terminal normally prints logs when there is a
  window too. The REPL still works from the terminal while using the window for
  rendering if the console is enabled.
- [ ] **Support for games with TUIs** - The REPL is designed to work as a sort of
  sidecar to the normal terminal output, so _in theory_ it should be compatible
  with games that use an alternate TUI screen. I don't know if it actually works.
- [ ] **Command history** - Use keybindings to navigate past commands
- [ ] **Customizable keybinds** - Allow the user to configure the REPL keybinds for
  all REPL controls, not just the toggle key.
- [ ] **Help text and command completion** - Use `clap`'s help text and completion
  features to provide a better REPL experience and allow for command discovery.

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
