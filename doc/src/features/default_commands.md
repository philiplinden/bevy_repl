# Built-in Commands

Each built-in command has its own feature flag. The `default_commands` feature
enables all of them, and it is on by default.

| Command | Aliases | Description | Feature Flag | Default |
| --- | --- | --- | --- | --- |
| [quit](#quit) | `q`, `exit` | Gracefully terminate the application | `quit` | `true` |
| [help](#help) | `h` | Show available commands | `help` | `true` |
| [clear](#clear) | None | Clear the screen | `clear` | `true` |

## quit

**Usage:** `quit`

**Aliases:** `q`, `exit`

`quit` gracefully terminates the application by sending an `AppExit` message.
This is the preferred way to exit a Bevy application. Unlike a simple `Ctrl+C`
or `SIGINT`, sending `AppExit` ensures that all of the application's resources
are cleaned up properly, including the REPL.

The REPL restores the terminal when the app exits, so you can build your own
quit command if you want. The important thing is that the REPL puts the
terminal in "raw mode", and the cleanup ensures that raw mode is disabled when
the app exits. Otherwise, the terminal may behave in unexpected ways even after
the app has exited.

## help

**Usage:** `help`

**Aliases:** `h`

Lists the commands registered with the REPL.

## clear

**Usage:** `clear`

**Aliases:** None

Clears the terminal screen.
