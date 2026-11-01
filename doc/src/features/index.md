# Feature Flags

| Feature Flag | Description | Default |
| --- | --- | --- |
| `derive` | Support clap's derive pattern for REPL commands | `true` |
| `default_commands` | Enable all built-in commands | `true` |
| `quit` | Enable the `quit` command | `true` (included in `default_commands`) |
| `help` | Enable the `help` command | `true` (included in `default_commands`) |
| `clear` | Enable the `clear` command | `true` (included in `default_commands`) |

Theoretically all clap features are supported, but only `derive` has been
tested. Override the `clap` features in your `Cargo.toml` to enable or disable
additional features at your own risk.

To turn off a built-in command, disable default features and list the ones you
want:

```toml
[dependencies]
bevy_repl = { git = "https://github.com/philiplinden/bevy_repl", default-features = false, features = ["derive", "quit", "help"] }
```

See [Built-in Commands](default_commands.md) for what each command does.
