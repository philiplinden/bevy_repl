# Known Issues & Limitations

<!-- toc -->

Known rough edges and limitations (see README for latest details):

- Built-in `help` and `clear` commands are not yet implemented.
- Ctrl+Enter and other advanced key combinations do not work.
- Directly modifying the terminal can be fragile if raw mode isn't restored.

**Tips:**

- Place your input event reader system before `bevy_repl::ReplSet::Pre` if you need to read inputs while REPL is enabled.
- If the terminal state is left odd after an abnormal exit, restart your
  terminal.
- If you are on Windows, use the REPL with `bevy_ratatui` added too.
  (See the `examples/alt_screen.rs` example.)

## Built-in `help` and `clear` commands are not yet implemented
I have `help` and `clear` implemented as placeholders. I don't consider this
crate to be feature-complete until these are implemented.

## Terminal behavior is inconsistent between Windows and Linux
The input buffer and cursor behavior is inconsistent between Windows and Linux.
On Linux, the cursor is always visible and input appears in the buffer as it is
typed. On Windows, the cursor and input buffer are not visible while typing. The
buffer is clearly interpreted as normal, but the user can't see it.

Interestingly, the cursor and input buffer are visible while typing in the
prompt when using the `bevy_ratatui` crate in conjunction with `bevy_repl`.

## Keybinds with modifier keys are not reliably detected
This might be related to not using Kitty protocol.
