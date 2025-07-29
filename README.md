# bevy_repl

Add an interactive REPL to a headless Bevy app using `clap` for command parsing
and `rustyline` for terminal input.

This crate exists because I wanted to use
[makspll/bevy-console](https://github.com/makspll/bevy-console) but in headless
mode, and takes heavy inspiration from that crate.

**Warning**: This is my first public Bevy plugin, and I vibe-coded a large part
of it. You have been warned.

**Current Limitation**: Commands that need to read from the Bevy `World`
(inspecting entities, components, resources) are not yet supported due to Bevy
ECS system parameter conflicts. Only commands that use `Commands` for spawning
entities, sending events, and basic operations currently work. See
[doc/DESIGN.md](doc/DESIGN.md) for technical details.

## Built-in Commands

| Command | Description |
| --- | --- |
| `close` | Close the REPL but do not exit the application |
| `quit` | Gracefully terminate the application |

## Usage

Custom commands can be added to the REPL by implementing the
`ReplCommand` trait, which allows you to register a `clap` command with the
REPL. Once the command is registered, it can be executed from the REPL.

The REPL can be toggled with a key binding. By default, the REPL is always
enabled. When the REPL is disabled, the prompt will not be shown.

```rust
use bevy::prelude::*;
use bevy_repl::{ReplPlugin, ReplConfig, ReplCommandRegistration, ReplCommand, ReplResult};

fn main() {
    // Option 1: Use default configuration
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::default())
        .add_repl_command::<CustomGameCommand>()
        .run();
}

// Option 2: Use custom configuration (requires `custom-history-file` feature)
fn main_with_config() {
    let config = ReplConfig::new()
        .with_prompt("game> ")
        .with_history_file(".my_game_history") // Custom history file
        .with_toggle_key(KeyCode::F1); // Toggle the REPL with F1

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .add_repl_command::<CustomGameCommand>()
        .run();
}

#[derive(Default)]
struct CustomGameCommand;

impl ReplCommand for CustomGameCommand {
    fn name(&self) -> &'static str { "spawn_player" }
    
    fn command(&self) -> clap::Command {
        clap::Command::new("spawn_player")
            .about("Spawns a player entity")
            .arg(clap::Arg::new("name").required(true))
    }
    
    fn execute(&self, commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let name = matches.get_one::<String>("name").unwrap();
        commands.spawn(PlayerBundle::new(name.clone()));
        Ok(format!("Spawned player: {}", name))
    }
}
```

The REPL will then be available in the terminal as a prompt
shown below the game's log messages when you run your game from the terminal.

```shell
INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL
game>
```

Enter commands in the REPL to interact with the game. The REPL will display the
output of the command in the terminal. For example, the `tree` command will list
all entities with components as a tree.

```shell
game> tree
```

```shell
INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL
game> tree
Entity 0:
  - bevy_core::name::Name
  - bevy_transform::components::transform::Transform
game>
```

## Configuration

The `ReplConfig` allows you to customize the REPL behavior. See
[doc/DESIGN.md](doc/DESIGN.md) for more details.

```rust
let config = ReplConfig::new()
    .with_prompt("game> ")
    .with_history_file(".my_app_history"); // Separate history for your app
```

By default, `rustyline` saves command history to `~/.rustyline_history`. You can
customize this by passing a custom history file to `ReplConfig`. This allows
different Bevy applications to have separate command histories.

```rust
let config = ReplConfig::new()
    .with_history_file(".my_game_history"); // App-specific history file
```

## Features

This plugin uses `clap` for command parsing and `rustyline` for terminal input,
history, and familiar terminal key bindings. Write custom commands with `clap`
and register them with the REPL to interact with ECS systems, components, and
resources from the terminal. Search your command history, tab complete commands,
and navigate your command history with the usual terminal key bindings.

The REPL runs with the game loop but operates in your terminal, so you can use
it on any platform that `rustyline` supports. It runs on a separate thread so as
to not block the game loop.

The following terminal actions are supported by `bevy_repl` through `rustyline`:

| Action | Description |
| --- | --- |
| `Ctrl-A, Home` | Move cursor to the beginning of line |
| `Ctrl-B, Left` | Move cursor one character left |
| `Ctrl-E, End` | Move cursor to end of line |
| `Ctrl-F, Right` | Move cursor one character right (or complete hint if cursor is at the end of line) |
| `Ctrl-H, Backspace` | Delete character before cursor |
| `Shift-Tab` | Previous completion |
| `Ctrl-I, Tab` | Next completion |
| `Ctrl-K` | Delete from cursor to end of line |
| `Ctrl-L` | Clear screen |
| `Ctrl-N, Down` | Next match from history |
| `Ctrl-P, Up` | Previous match from history |
| `Ctrl-R` | Reverse Search history (Ctrl-S forward, Ctrl-G cancel) |
| `Ctrl-V (unix)` | Insert any special character without performing its associated action (#65) |
| `Ctrl-V (windows)` | Paste from clipboard |
| `Ctrl-Y` | Paste from Yank buffer |

### Optional Features

| Feature | Description | Default |
| --- | --- | --- |
| `dev` | Enable dynamic linking for faster compilation | `true` |
| `custom-history-file` | Change the default history file for persistent command history saved across sessions (requires `rustyline/with-file-history` feature) | `false` |

This crate uses `clap` for command parsing and `rustyline` for terminal input
and history. Both of these dependencies are included with default features.
You can enable or disable features by configuring these dependencies in your
`Cargo.toml` alongside the `bevy_repl` dependency.

```toml
[dependencies]
bevy_repl = "0.1"
clap = { version = "4.5", features = ["derive", "suggestions", "color"] }
rustyline = { version = "16.0", features = ["with-file-history", "with-dirs"] }
```

**Available clap features**: `derive`, `suggestions`, `color`, `help`, `usage`, `env`, `wrap_help`
**Available rustyline features**: `with-file-history`, `with-dirs`, `with-fuzzy`, `with-custom-bindings`

See the [clap](https://docs.rs/clap) and [rustyline](https://docs.rs/rustyline)
documentation for complete feature lists and descriptions.

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
