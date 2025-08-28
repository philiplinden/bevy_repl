# Aspirations


- [x] **Derive pattern** - Describe commands with clap's derive pattern. 
- [x] **Support for games with rendering and windowing** - The REPL is designed to
  work from the terminal, but the terminal normally prints logs when there is a
  window too. The REPL still works from the terminal while using the window for
  rendering if the console is enabled.
- [x] **Printing to stdout** - The REPL should print to stdout instead of the
  TUI screen unless the user explicitly enables a TUI context that uses the
  alternate screen.
- [ ] **Toggleable** - The REPL is disabled by default and can be toggled. When
  disabled, the app runs normally in the terminal, no REPL systems run, and the
  prompt is hidden.
- [ ] **Scrollable terminal output** - The terminal output on the TUI screen
  should scroll to show past messages like a normal terminal screen printing to
  stdout.
- [ ] **Support for games with TUIs** - The REPL is designed to work as a sort of
  sidecar to the normal terminal output, so _in theory_ it should be compatible
  with games that use an alternate TUI screen. I don't know if it actually
  works, probably only with the minimal renderer or perhaps a custom renderer.
- [ ] **Customizable keybinds** - Allow the user to configure the REPL keybinds for
  all REPL controls, not just the toggle key.
- [ ] **Command history** - Use keybindings to navigate past commands and insert
  them in the prompt buffer.
- [ ] **Help text and command completion** - Use `clap`'s help text and completion
  features to provide a better REPL experience and allow for command discovery.
- [ ] **Customizable prompt** - Allow the user to configure the REPL prompt for
  all REPL controls, not just the toggle key.
