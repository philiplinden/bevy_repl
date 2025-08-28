# Terminal Screens

Ratatui TUIs often use an alternate screen (separate from stdout). Bevy REPL favors a "partial-TUI" that renders the prompt while keeping stdout usable.

- When REPL is active, the terminal runs in raw mode and prints to stdout.
- Prefer `bevy_repl::repl_println!` over `println!` while REPL is active to avoid cursor/newline glitches.
- If you enable a full alternate screen via `bevy_ratatui::RatatuiPlugins`, REPL still works but output behavior changes.

`repl_println!` ensures safe, consistent output:

```rust
fn on_ping(_t: Trigger<Ping>) {
    bevy_repl::repl_println!("Pong");
}

fn instructions() {
    bevy_repl::repl_println!();
    bevy_repl::repl_println!("Welcome to the Bevy REPL!");
}
```
