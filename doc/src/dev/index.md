# Development

This chapter contains developer notes, to-dos, known issues, and other
information for those who want to contribute to the crate.

## TUI Mode

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

## Non-TUI Mode

Starting in v0.5.0, the REPL can be used in non-TUI mode where the prompt
and logs are printed to stdout together with the builtin log handler. This
mode is lightweight and intended to be a minimal layer on top of the Bevy app.

Here, _lightweight_ means that the REPL adds minimal dependencies and is purely
additive with no disruptions to the normal logging behavior.
