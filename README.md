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
