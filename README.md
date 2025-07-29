# bevy_repl

Add an interactive REPL to a headless Bevy app using `clap` for command parsing
and `rustyline` for terminal input.

This crate exists because I wanted to use
[makspll/bevy-console](https://github.com/makspll/bevy-console) but in headless
mode, and takes heavy inspiration from that crate.

This is my first public Bevy plugin, and I vibe-coded a large part of it.

## Features

This plugin uses the `rustyline` library to handle terminal input, history, and
suggestions.

RustyLine supports the following platforms:

- Unix (tested on FreeBSD, Linux and macOS)
- Windows
  - cmd.exe
  - Powershell

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
| `derive` | Enable derive macros for clap and rustyline | `false` |
| `suggestions` | Enable command suggestions with `clap` | `true` |
| `color` | Enable colored output with `clap` | `true` |
| `help` | Enable help text with `clap` | `true` |
| `usage` | Enable usage information with `clap` | `true` |
| `env` | Enable environment variable support with `clap` | `true` |
| `wrap_help` | Enable help text wrapping with `clap` (requires `help` feature) | `true` |
| `diagnostics` | Enable Bevy system information inspection | `false` |
| `case-insensitive` | Enable case-insensitive history search with `rustyline` | `true` |
| `fuzzy-search` | Enable fuzzy history search with `rustyline` | `true` |
| `save-history` | Enable file history with `rustyline` | `true` |
| `dir-completions` | Enable directory completion with `rustyline` | `true` |
| `custom-bindings` | Enable custom key bindings with `rustyline` | `true` |

## Built-in Commands

| Command | Description |
| --- | --- |
| `help` | List all available commands |
| `quit` | Gracefully terminate the application |
| `tree` | List entities with components as a tree |
| `sysinfo` | Show system information (requires `diagnostics` feature) |

## Usage

Add `ReplPlugin` with `ReplConfig` to customize the REPL behavior.

```rust
use bevy::prelude::*;
use bevy_repl::{ReplPlugin, ReplConfig, ReplCommandRegistration, ReplCommand, ReplResult};

fn main() {
    // Option 1: Use default configuration
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::default())
        .register_command::<CustomGameCommand>()
        .run();
}

// Option 2: Use custom configuration
fn main_with_config() {
    let config = ReplConfig::new()
        .with_prompt("game> ")

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .register_command::<CustomGameCommand>()
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
    
    fn execute(&self, world: &mut World, matches: &clap::ArgMatches) -> ReplResult<String> {
        let name = matches.get_one::<String>("name").unwrap();
        world.spawn(PlayerBundle::new(name.clone()));
        Ok(format!("Spawned player: {}", name))
    }
}
```

## Configuration

The `ReplConfig` allows you to customize the REPL behavior. See
[doc/DESIGN.md](doc/DESIGN.md) for more details.

```rust
let config = ReplConfig::new()
    .with_prompt("game> ")
```

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
