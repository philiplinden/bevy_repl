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

### 1. Unified `Repl` Resource & Terminal Lifecycle
Terminal lifecycle and raw-mode management are consolidated directly onto the single `Repl` resource, eliminating redundant resources (`ReplTerminal`, `ReplPrompt`, `ReplPromptConfig`):

```rust
#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub prompt_symbol: String,
    pub buffer: String,
    pub cursor_pos: usize,
    pub toggle_key: Option<KeyCode>,
    pub commands: HashMap<String, Box<dyn CommandParser>>,
}

impl Repl {
    pub fn init_terminal() -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut out = std::io::stdout();
        if let Ok((_, rows)) = crossterm::terminal::size() {
            let bottom = rows.saturating_sub(1);
            let _ = write!(out, "\x1B[1;{}r", bottom);
            let _ = out.flush();
        }
        Ok(())
    }

    pub fn restore_terminal() -> std::io::Result<()> {
        use std::io::Write;
        let mut out = std::io::stdout();
        let (_, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let prompt_row = rows.saturating_sub(1);

        // 1. Clear prompt line
        let _ = crossterm::queue!(
            out,
            crossterm::cursor::MoveTo(0, prompt_row),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        );
        // 2. Reset scroll region back to full screen
        let _ = write!(out, "\x1B[r");
        // 3. Move cursor to bottom row & show cursor
        let _ = crossterm::queue!(out, crossterm::cursor::MoveTo(0, prompt_row), crossterm::cursor::Show);
        let _ = write!(out, "\r\n");
        let _ = out.flush();
        // 4. Disable raw mode
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_terminal() -> std::io::Result<()> {
        use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
        let mut out = std::io::stdout();
        execute!(out, Clear(ClearType::All), Clear(ClearType::Purge), MoveTo(0, 0))?;
        Ok(())
    }
}
```

### 2. Safety Hooks (Panic & SIGINT)
`install_safety_hooks()` wraps `std::panic::set_hook` and `ctrlc::set_handler` to guarantee `Repl::restore_terminal()` executes before process termination:

```rust
pub fn install_safety_hooks() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = Repl::restore_terminal();
        default_hook(panic_info);
    }));

    let _ = ctrlc::set_handler(move || {
        let _ = Repl::restore_terminal();
        std::process::exit(0);
    });
}
```

### 3. Synchronous Lifecycle System
`handle_repl_lifecycle` executes state transitions immediately without deferred command buffering:

```rust
pub fn handle_repl_lifecycle(
    mut reader: MessageReader<ReplLifecycleEvent>,
    mut repl: ResMut<Repl>,
) {
    for event in reader.read() {
        let should_enable = match event {
            ReplLifecycleEvent::Enable => true,
            ReplLifecycleEvent::Disable => false,
            ReplLifecycleEvent::Toggle => !repl.enabled,
        };

        if should_enable && !repl.enabled {
            repl.enabled = true;
            let _ = Repl::init_terminal();
        } else if !should_enable && repl.enabled {
            repl.enabled = false;
            repl.clear_buffer();
            let _ = Repl::restore_terminal();
        }
    }
}
```
