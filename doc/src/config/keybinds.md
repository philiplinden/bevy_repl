# Keybinds

The following keybinds control the REPL's input buffer cursor.

| Key | Action |
| --- | --- |
| Enter | Submit command |
| Esc | Clear input buffer |
| Left/Right | Move cursor |
| Home/End | Jump to start/end |
| Backspace | Delete before cursor |
| Delete | Delete at cursor |
| Ctrl+C | Terminate app |

**Note:** Ctrl+C behaves like a normal terminal interrupt because Bevy REPL
implements a hook to handle `SIGINT` (Ctrl+C) interrupts in addition to Bevy's
`AppExit` event to restore the terminal state (disable "raw mode") on exit. This
is baked into the REPL plugin and doesn't require any additional setup, so
Ctrl+C still works even if the built-in quit command is disabled.

Keybinds for the input buffer are not yet customizable (_see
[Known Issues & Limitations](../dev/known_issues.md)_).
