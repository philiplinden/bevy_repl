# Terminal Lifecycle and Panic Recovery

Type: grilling
Status: resolved

## Question

How should the terminal's raw mode and screen restoration be structured as a Bevy resource/plugin so that `disable_raw_mode()` and DECSTBM reset (`\x1B[r`) are guaranteed to execute on normal exit (`AppExit`), unexpected error, and process panic?

Specifically:
1. What is the lifecycle and ownership model of the `ReplTerminal` resource?
2. How do `std::panic::set_hook` and `color_eyre` integrate without colliding with Bevy's default error/panic reporting?
3. What happens when the REPL is disabled dynamically at runtime vs app shutdown?

## Answer

### 1. `ReplTerminal` RAII Resource & Drop Semantics
Create a dedicated `ReplTerminal` resource that owns terminal raw mode. Its `Drop` implementation calls `ReplTerminal::restore()` to ensure that normal app exit, system panic, or resource removal resets the terminal.

```rust
#[derive(Resource)]
pub struct ReplTerminal {
    pub raw_mode_enabled: bool,
}

impl Drop for ReplTerminal {
    fn drop(&mut self) {
        let _ = Self::restore();
    }
}

impl ReplTerminal {
    pub fn init() -> std::io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self { raw_mode_enabled: true })
    }

    pub fn restore() -> std::io::Result<()> {
        use std::io::Write;
        let mut out = std::io::stdout();
        // 1. Reset DECSTBM scroll region to full window
        let _ = write!(out, "\x1B[r");
        // 2. Ensure cursor is visible
        let _ = crossterm::execute!(out, crossterm::cursor::Show);
        // 3. Move to fresh line to avoid prompt overlap
        let _ = write!(out, "\r\n");
        let _ = out.flush();
        // 4. Disable raw mode
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}
```

### 2. Standard Panic Hook Wrapper
Keep panic handling simple and dependency-free using `std::panic::take_hook` and `std::panic::set_hook`. When a panic occurs, `ReplTerminal::restore()` is executed before forwarding to the default hook, preventing stair-stepped backtraces and leaving the terminal in a clean state.

```rust
pub fn install_terminal_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = ReplTerminal::restore();
        default_hook(panic_info);
    }));
}
```

### 3. Dynamic Runtime Toggle Lifecycle
- When `ReplLifecycleEvent::Enable` is triggered, `enable_raw_mode()` is called and the `ReplTerminal` resource is inserted.
- When `ReplLifecycleEvent::Disable` is triggered, `ReplTerminal::restore()` is called and the resource is removed (or `raw_mode_enabled` set to false).
- App shutdown (`AppExit`) removes `ReplTerminal`, triggering `drop()` to clean up.
