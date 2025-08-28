# Default commands
Enable built-in commands with feature flags. Each command is enabled separately
by a feature flag. Use the `default_commands` feature to enable all built-in
commands.

 Command | Aliases | Description | Feature Flag | Default |
| --- | --- | --- | --- | --- |
| [quit](#quit) | `quit`, `q`, `exit` | Gracefully terminate the application | `quit` | `true` |
| [help](#help) | `help` | Show available commands | `help` | `true` |
| [clear](#clear) | `clear` | Clear the screen | `clear` | `false` |

## quit

**Usage:** `quit`

**Aliases:** `q`, `exit`

`quit` gracefully terminates the application by sending an `AppExit` event in
ECS. This is the preferred way to exit a Bevy application. Unlike a simple
`Ctrl+C` or `SIGINT`, sending the `AppExit` event ensures that all of the
application's resources are cleaned up properly, including the REPL.

Bevy REPL has an observer that restores the terminal state when the `AppExit`
event is read, so you can build your own quit command if you want. The important
thing is that the REPL modifies the terminal state (it puts the terminal in
"raw mode") and the cleanup ensures that "raw mode" is disabled when the app
exits. If raw mode is not disabled, the terminal may behave in unexpected ways
even after the app has exited.

## help

**Usage:** `help`

**Aliases:** None

Shows all commands available to the REPL. (Not implemented)

## clear

**Usage:** `clear`

**Aliases:** None

Clears the screen. (Not implemented)
