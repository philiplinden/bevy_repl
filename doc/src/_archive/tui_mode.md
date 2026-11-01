# TUI Mode (archived)

_Archived 2026-11-01 from `dev/index.md` and `dev/known_issues.md`. This describes the ratatui-era REPL (v0.1–v0.5)._

### TUI Mode

TUI (terminal user interface) mode is the original approach to the REPL,
where the prompt and logs are captured and redirected away from the builtin
log handler to the TUI renderer.

The TUI adds large dependencies and can break the normal logging behavior.
On the other hand, it is togglable at runtime and supports custom keybinds.

### Feature Wishlist

- [x] **Derive pattern** (_Added in v0.3.0_) - Describe commands with clap's
  derive pattern.
- [x] **Support for games with rendering and windowing** (_Added in v0.3.0_) -
  The REPL is designed to work from the terminal, but the terminal normally
  prints logs when there is a window too. The REPL still works from the terminal
  while using the window for rendering if the console is enabled.
- [x] **Printing to stdout** (_Added in v0.4.0_) - The REPL should print to
  stdout instead of the TUI screen unless the user explicitly enables a TUI
  context that uses the alternate screen.
- [x] **Toggleable** (_Added in v0.4.1_) - The REPL is disabled by default and
  can be toggled. When disabled, the app runs normally in the terminal, no REPL
  systems run, and the prompt is hidden.
- [ ] **Scrollable TUI output** - The terminal output on the TUI screen should
  scroll to show past messages like a normal terminal screen printing to stdout.
- [ ] **Support for games with TUIs** - The REPL is designed to work as a sort
  of sidecar to the normal terminal output, so _in theory_ it should be
  compatible with games that use an alternate TUI screen. I don't know if it
  actually works, probably only with the minimal renderer or perhaps a custom
  renderer.
- [x] **Customizable keybinds** (_Added in v0.4.1_) - Allow the user to
  configure the REPL keybinds for all REPL controls, not just the toggle key.
- [ ] **Command history** - Use keybindings to navigate past commands and insert
  them in the prompt buffer.
- [ ] **Help text and command completion** - Use `clap`'s help text and
  completion features to provide a better REPL experience and allow for command
  discovery.
- [ ] **Customizable prompt** - Allow the user to configure the REPL prompt for
  all REPL controls, not just the toggle key.

### Non-TUI Mode

Starting in v0.5.0, the REPL can be used in non-TUI mode where the prompt
and logs are printed to stdout together with the builtin log handler. This
mode is lightweight and intended to be a minimal layer on top of the Bevy app.

Here, _lightweight_ means that the REPL adds minimal dependencies and is purely
additive with no disruptions to the normal logging behavior.

## Known issues (TUI era)

### TUI Mode

- Built-in `help` and `clear` commands are not yet implemented.
- Ctrl+Enter and other advanced key combinations do not work.
- Directly modifying the terminal can be fragile if raw mode isn't restored.

**Tips:**

- Place your input event reader system before `bevy_repl::ReplSet::Pre` if you need to read inputs while REPL is enabled.
- If the terminal state is left odd after an abnormal exit, restart your
  terminal.
- If you are on Windows, use the REPL with `bevy_ratatui` added too.
  (See the `examples/alt_screen.rs` example.)

### Built-in `help` and `clear` commands are not yet implemented
I have `help` and `clear` implemented as placeholders. I don't consider this
crate to be feature-complete until these are implemented.

### Terminal behavior is inconsistent between Windows and Linux
The input buffer and cursor behavior is inconsistent between Windows and Linux.
On Linux, the cursor is always visible and input appears in the buffer as it is
typed. On Windows, the cursor and input buffer are not visible while typing. The
buffer is clearly interpreted as normal, but the user can't see it.

Interestingly, the cursor and input buffer are visible while typing in the
prompt when using the `bevy_ratatui` crate in conjunction with `bevy_repl`.

### Keybinds with modifier keys are not reliably detected
This might be related to not using Kitty protocol.
