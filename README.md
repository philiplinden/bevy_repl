# bevy_repl

Add an interactive REPL to a headless Bevy app using `clap` for command parsing.

This crate exists because I wanted to use
[makspll/bevy-console](https://github.com/makspll/bevy-console) but in headless
mode, and takes heavy inspiration from that crate.

This is my first public Bevy plugin, and I vibe-coded a large part of it.

## Features

| Feature | Description | Default |
| --- | --- | --- |
| `diagnostics` | Enable system information commands | `true` |
| `history` | Enable command history | `true` |
| `suggestions` | Enable predictive search suggestions | `false` |

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
use bevy_repl::command::{HelpCommand, QuitCommand, TreeCommand, SysInfoCommand};

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
        .with_auto_headless(true)
        .with_headless_args(vec!["--repl".to_string()])
        .with_headless_env_var(Some("GAME_REPL".to_string()));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .register_command::<HelpCommand>()
        .register_command::<QuitCommand>()
        .register_command::<TreeCommand>()
        .register_command::<SysInfoCommand>()
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

## Configuration

The `ReplConfig` allows you to customize the REPL behavior:

### Configuration Options

- **`prompt`** - The prompt string to display (default: `"> "`)
- **`enabled`** - Whether to enable the REPL on startup (default: `true`)
- **`auto_headless`** - Whether to start in headless mode automatically (default: `false`)
- **`headless_args`** - Custom command line arguments to detect headless mode (default: `["--headless", "--server", "--no-gui"]`)
- **`headless_env_var`** - Environment variable to check for headless mode (default: `Some("BEVY_HEADLESS")`)

### Configuration Methods

```rust
let config = ReplConfig::new()
    .with_prompt("game> ")
    .with_enabled(true)
    .with_auto_headless(true)
    .with_headless_args(vec!["--repl".to_string()])
    .with_headless_env_var(Some("GAME_REPL".to_string()));
```

## Built-in Commands

The crate provides several built-in commands that you can register:

### `help`
Lists all available commands or shows detailed help for a specific command.

```bash
help              # List all commands
help spawn_player # Show help for spawn_player command
```

### `quit` / `exit`
Gracefully shuts down the application.

```bash
quit   # Exit the application
exit   # Same as quit
q      # Short alias for quit
```

### `tree`
Lists entities with their components in a tree structure.

```bash
tree        # Show all entities
tree 123    # Show specific entity by ID
```

### `sysinfo`
Shows system information including entity count, component count, and memory usage.

```bash
sysinfo
```

## Example Output

Running `tree` might show:
```
Entity Tree:
Entity 0:
  - bevy_core::name::Name
  - bevy_transform::components::transform::Transform

Entity 1:
  - bevy_core::name::Name
  - bevy_transform::components::transform::Transform
  - Health

Entity 2:
  - bevy_core::name::Name
  - bevy_transform::components::transform::Transform
  - bevy_core::camera::camera::Camera3d
```

Running `sysinfo` might show:
```
System Information:
==================

Total Entities: 3
Total Components: 156
Total Resources: 42
Total Archetypes: 3
Approximate Memory Usage: 2048 bytes
```
```

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
