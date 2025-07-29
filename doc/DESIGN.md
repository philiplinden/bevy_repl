# Design

The REPL is implemented as a Bevy plugin. It is responsible for:

- Receiving input from the user via the terminal
- Parsing the input into a command using `clap`
- Executing the command using the Bevy ECS
- Displaying the output in the terminal with other Bevy log messages

The REPL is meant to be an alternative to [makspll/bevy-console] for Bevy apps
that don't need a GUI but still want a console for debugging and development.

[makspll/bevy-console]: https://github.com/makspll/bevy-console

## User Experience

A developer adds the REPL plugin to their Bevy app and configures it with a
config resource. Custom commands can be added to the REPL by implementing the
`ReplCommand` trait, which allows you to register a `clap` command with the
REPL.

```rust
fn main() {
    let config = ReplConfig::new()
        .with_prompt("game> ")

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .add_repl_command::<CustomGameCommand>()
        .run();
}
```

The REPL will then be available in the terminal as a prompt
shown below the game's log messages.

```shell
INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL
game>
```

The developer can then type commands to interact with the game. The REPL will
display the output of the command in the terminal.

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

To add or remove features from `clap` or `rustyline`, you can enable or disable
features in your `Cargo.toml` file alongside the `bevy_repl` dependency.

```toml
[dependencies]
bevy_repl = "0.1.0"
clap = { version = "4.5", features = ["derive", "suggestions", "color"] }
rustyline = { version = "16.0", features = ["with-file-history", "with-dirs"] }
```

## Design Decisions

### Why a separate thread for input handling?

**Problem:** Bevy's main thread runs the game loop and ECS systems. If we tried to
read user input directly in the main thread, it would:

- Block the entire game when waiting for user input
- Prevent the game from running at consistent frame rates
- Create a poor user experience

**Solution:** Move input handling to a separate thread that can block safely while
the main game continues running.

### Why use `clap` and `rustyline`?

The REPL uses two key libraries for handling user interaction:

`clap` handles command parsing by providing a robust, well-documented argument
parser with features like:

- Help message generation
- Subcommand support
- Argument validation
- Strong community support

`rustyline` manages terminal input with capabilities including:

- Command history
- Tab completion
- Syntax highlighting
- Wide community adoption

## Built-in Commands

### `tree`

The `tree` command lists all entities in the world with their components.

Running `tree` might show:

```shell
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

### `sysinfo`

The `sysinfo` command shows information about the system, including the number of
entities, components, resources, memory usage, and other system information
provided by the `diagnostics` Bevy feature.

Running `sysinfo` might show:

```shell
System Information:
==================

Total Entities: 3
Total Components: 156
Total Resources: 42
Approximate Memory Usage: 2048 bytes
```

## Future Features

- [ ] Add command suggestions with `trie-rs` similar to the implementation in
  `bevy-console`.
- [ ] Add a `clear` command to clear the terminal.
- [ ] Add a `history` command to show the command history.
- [ ] Add a `clear-history` command to clear the command history.
