# Known Issues & Limitations

<!-- toc -->

## Windows and Unix behave differently

The terminal backend (`crossterm`) works differently on Windows and on Unix.
macOS uses the same Unix backend as Linux, so expect it to behave like Linux.
Known differences:

- **Raw mode and newlines.** On Unix, raw mode also turns off output
  processing, so a bare `\n` moves down a line without returning to column 0
  and output "stair-steps". On Windows, raw mode leaves output alone. Print
  with `repl_println!`, which writes `\r\n`.
- **Key release events.** Windows reports key releases as well as presses;
  Unix reports presses only. The REPL only acts on presses and repeats.
- **Prompt visibility.** In the ratatui era, the prompt text and cursor were
  invisible while typing on Windows. This hasn't been rechecked since the
  crossterm rewrite.

If a bug seems to come and go, check whether you switched operating systems or
terminals between runs.

## Code that doesn't match the docs yet

These are gaps between the current code and the behavior the book describes.

- **Logging isn't additive yet.** Most examples call
  `DefaultPlugins.adapt_for_repl()`, which disables `LogPlugin` so the REPL can
  install its own log writer. Log lines written while the user types can garble
  the prompt.
- **Toggle machinery is still in the code.** The `F3` toggle, the `enabled`
  flag, and the lifecycle actions are left over from earlier versions. Toggling
  is back in the plan as of the
  [2026-11-02 devlog](../devlog/03-crossterm-rewrite/2026-11-02.md), so this
  code may be reused.

- **`MinimalPlugins` is untested.** Every example uses `DefaultPlugins`. The
  system that keeps typed keys away from the game reads Bevy's keyboard input
  resources, which `MinimalPlugins` doesn't add.

## Bugs

- **Non-ASCII input panics.** The buffer is indexed by bytes rather than
  characters, so inserting or deleting around a multi-byte character can panic.
  The prompt's cursor column has the same byte/character mix-up.
- **Unknown arguments are silent.** If a command gets an argument it doesn't
  recognize, the parse error isn't shown to the user.
- **Ctrl+C always exits.** The keymap binds Ctrl+C to clear the line, but the
  exit check runs first, so that binding never fires.
- **Shift+Enter inserts a newline** into the single-row prompt.
- **`help` output is unordered.** Commands are listed in hash-map order, and
  aliases appear as separate entries.

## Tips

- If the terminal is left in an odd state after an abnormal exit, run `reset`
  (Unix) or restart the terminal.
