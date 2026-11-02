# bevy_repl

A Bevy plugin that provides a Read-Eval-Print Loop (REPL) interface for
interactive command input.

The `ReplPlugins` plugin group adds a REPL to the terminal console while your
Bevy application runs, allowing users to enter commands and interact with the
ECS at runtime with or without a renderer.

```rust
use bevy::prelude::*;
use bevy_repl::ReplPlugins;

fn main() {
    App::new().add_plugins((DefaultPlugins, ReplPlugins));
}
```

![Made with VHS](https://vhs.charm.sh/vhs-6kUt4mnyvUcmbpVfWzHx4s.gif)

BevyREPL is powered by `clap` for command parsing and `crossterm` for
terminal input and output. The plugin adds a text input area below the terminal
output for interaction even in headless mode.

- A one-line prompt pinned below normal terminal output, with no TUI
- Command parsing and CLI features from `clap`
- Observer-based command execution system with full Bevy ECS access for both
  read and write operations
- Bevy logs and `println!` output keep printing above the prompt, with no
  changes to `LogPlugin` or `DefaultPlugins`
- Works in terminal with headless and windowed apps
- Built-in `quit`, `help`, and `clear` commands

The REPL is designed as an alternative to
[makspll/bevy-console](https://github.com/makspll/bevy-console) for Bevy apps
that want a terminal-like console to modify the game at runtime without
implementing a full TUI, and something that works with or without a renderer.

_This is my first public Bevy plugin, and I vibe-coded a large part of it._ 
**_You have been warned_.**

| Version | Bevy | Notes |
| --- | --- | --- |
| 1.0.0 (unreleased) | 0.20 | Rewrite on `crossterm` without `ratatui`. Work in progress on `main`. |
| 0.5.0 | 0.19.0 | Compatibility update for [Bevy 0.19.0](https://github.com/bevyengine/bevy/releases/tag/0.19.0). Depends on [ratatui/bevy_ratatui#98](https://github.com/ratatui/bevy_ratatui/pull/98) |
| 0.4.1 | 0.16.1 | Better docs: [philiplinden.github.io/bevy_repl](https://philiplinden.github.io/bevy_repl/) |
| 0.4.0 | 0.16.1 | Removed the "pretty" renderer in favor of getting simple prompt features working. Changed the interface slightly. This is a breaking change! See [examples](https://github.com/philiplinden/bevy_repl/tree/main/examples) for help. |
| 0.3.0 | 0.16.1 | First release. Supports `derive` feature. Only `quit` built-in command is implemented. Includes a "pretty" renderer for fancy prompt styling, but it doesn't work very well. |

## Features

Theoretically all clap features are supported, but I have only tested `derive`.
Override the `clap` features in your `Cargo.toml` to enable or disable
additional features at your own risk.

| Feature Flag | Description | Default |
| --- | --- | --- |
| `derive` | Support clap's derive pattern for REPL commands | `true` |
| `default_commands` | Enable all built-in commands | `true` |
| `quit` | Enable the `quit` command | `true` (included in `default_commands`) |
| `help` | Enable the `help` command | `true` (included in `default_commands`) |
| `clear` | Enable the `clear` command | `true` (included in `default_commands`) |

## Batteries-included setup

1.0 isn't on crates.io yet. To try the current work toward it, depend on this
repository directly:

```toml
[dependencies]
bevy = "0.20"
bevy_repl = { git = "https://github.com/philiplinden/bevy_repl" }
```

### ReplPlugins

```rust,no_run
use bevy::prelude::*;
use bevy_repl::ReplPlugins;

fn main() {
    App::new()
        // Headless with a stable frame time (60 FPS) - this is important!
        .add_plugins((
            DefaultPlugins
                .set(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_secs_f32(1.0/60.0)
                )),
            ReplPlugins,
        ))
        .run();
}
```

### ReplCommand

Input is parsed via `clap` commands and corresponding observer systems that
execute when triggered by the command.

1. Define a command type by deriving `Event` and implementing `ReplCommand`
   (or deriving it if you have the `derive` feature enabled).
2. Register the command with the app with `.add_repl_command::<YourCommand>()`.
3. Handle the command event with an observer: `.add_observer(on_command)`.

The REPL parses prompt input to a `YourCommand` event, where the fields are
the parsed arguments and options. Use observers to handle the event with full
ECS access.

## BevyREPL Book

The BevyREPL Book is a collection of docs and notes about the BevyREPL, how to
use it, and how it works under the hood.

The book is available at
[philiplinden.github.io/bevy_repl](https://philiplinden.github.io/bevy_repl/).

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
