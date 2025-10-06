**YES! This is totally achievable** and there are two ways to do it with crossterm:

## Option 1: "Sticky Prompt" Pattern (What You Described)

This keeps the prompt at the bottom while logs scroll above it:

```
[Game logs scrolling...]
Enemy spawned at (10, 20)
Player health: 85
Resource loaded: texture.png
                              ← logs scroll here
>> spawn enemy|               ← prompt STAYS here (cursor position)
```

### Implementation

```rust
// src/prompt/render.rs
use crossterm::{cursor, execute, queue, terminal, style::Print};

#[derive(Resource)]
pub struct PromptRenderer {
    prompt_line: u16,  // Which terminal row the prompt is on
    output_buffer: VecDeque<String>,  // Recent log lines
    max_output_lines: usize,
}

impl PromptRenderer {
    pub fn new() -> Self {
        let (_, rows) = terminal::size().unwrap();
        Self {
            prompt_line: rows - 1,  // Bottom row
            output_buffer: VecDeque::new(),
            max_output_lines: 100,
        }
    }

    /// Print a log line ABOVE the prompt
    pub fn print_output(&mut self, line: String) {
        self.output_buffer.push_back(line);
        if self.output_buffer.len() > self.max_output_lines {
            self.output_buffer.pop_front();
        }
        self.redraw_all();
    }

    /// Redraw everything: logs + prompt
    fn redraw_all(&self) {
        let mut stdout = io::stdout();
        
        // Save current cursor position
        queue!(stdout, cursor::SavePosition).unwrap();

        // Calculate how many lines of output we can show
        let visible_lines = self.prompt_line.saturating_sub(1) as usize;
        let start_idx = self.output_buffer.len().saturating_sub(visible_lines);

        // Draw output lines from top
        for (i, line) in self.output_buffer.iter().skip(start_idx).enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(0, i as u16),
                terminal::Clear(terminal::ClearType::CurrentLine),
                Print(line),
            ).unwrap();
        }

        // Restore cursor to prompt line
        queue!(stdout, cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
    }

    /// Render the prompt line (call every frame)
    pub fn render_prompt(&self, terminal_state: &TerminalState) {
        let mut stdout = io::stdout();

        // Always draw on the prompt line
        queue!(
            stdout,
            cursor::MoveTo(0, self.prompt_line),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(&terminal_state.prompt),
            Print(&terminal_state.buffer),
        ).unwrap();

        // Position cursor
        let cursor_x = (terminal_state.prompt.len() + terminal_state.cursor_pos) as u16;
        queue!(
            stdout,
            cursor::MoveTo(cursor_x, self.prompt_line),
        ).unwrap();

        stdout.flush().unwrap();
    }
}
```

### Usage in Systems

```rust
// Print logs system
fn handle_log_output(
    mut renderer: ResMut<PromptRenderer>,
    mut log_reader: MessageReader<LogEvent>,
) {
    for log in log_reader.read() {
        renderer.print_output(format!("[{}] {}", log.level, log.message));
    }
}

// Render prompt system (every frame)
fn render_repl_prompt(
    renderer: Res<PromptRenderer>,
    terminal_state: Res<TerminalState>,
) {
    renderer.render_prompt(&terminal_state);
}

// App setup
app.add_systems(Update, (
    handle_log_output,
    render_repl_prompt,
).chain());
```

### Visual Effect

```
INFO: Game started
DEBUG: Loading assets...
INFO: Player spawned at (0, 0)
WARN: Low memory
INFO: Enemy spawned           ← New log appears here
>> spawn boss|                ← Prompt stays here (cursor blinks at |)
```

When a new log comes in, it scrolls up and the prompt redraws at the bottom.

## Option 2: Alt-Screen Mode (Cleaner Separation)

Use crossterm's alternate screen for a true TUI where logs and prompt are completely separate regions:

```rust
// src/terminal.rs
pub struct TerminalLayout {
    output_area: Rect,   // Top region for logs
    prompt_area: Rect,   // Bottom region for prompt
}

pub fn setup_terminal(mut commands: Commands) -> Result {
    let mut stdout = io::stdout();
    
    // Enter alternate screen
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::EnableBlinking,
    )?;
    terminal::enable_raw_mode()?;

    let (cols, rows) = terminal::size()?;
    
    commands.insert_resource(TerminalLayout {
        output_area: Rect {
            x: 0,
            y: 0,
            width: cols,
            height: rows - 2,  // Leave 2 rows for prompt + separator
        },
        prompt_area: Rect {
            x: 0,
            y: rows - 1,
            width: cols,
            height: 1,
        },
    });

    Ok(())
}

pub fn render_split_view(
    layout: Res<TerminalLayout>,
    logs: Res<LogBuffer>,
    terminal_state: Res<TerminalState>,
) {
    let mut stdout = io::stdout();

    // Draw separator line
    queue!(
        stdout,
        cursor::MoveTo(0, layout.output_area.height),
        Print("─".repeat(layout.output_area.width as usize)),
    ).unwrap();

    // Draw logs in output area
    for (i, line) in logs.recent_lines().iter().enumerate() {
        if i >= layout.output_area.height as usize {
            break;
        }
        queue!(
            stdout,
            cursor::MoveTo(0, i as u16),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(line),
        ).unwrap();
    }

    // Draw prompt in prompt area
    queue!(
        stdout,
        cursor::MoveTo(0, layout.prompt_area.y),
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print(&terminal_state.prompt),
        Print(&terminal_state.buffer),
        cursor::MoveTo(
            (terminal_state.prompt.len() + terminal_state.cursor_pos) as u16,
            layout.prompt_area.y
        ),
    ).unwrap();

    stdout.flush().unwrap();
}
```

This gives you:

```
┌────────────────────────────────────┐
│ INFO: Game started                 │
│ DEBUG: Loading assets...           │
│ INFO: Player spawned               │
│ WARN: Low memory                   │
│ INFO: Enemy spawned                │ ← Scrollable log area
│                                    │
│ (scrolls independently)            │
├────────────────────────────────────┤ ← Separator
│ >> spawn boss|                     │ ← Fixed prompt line
└────────────────────────────────────┘
```

## 🎯 Which One Should You Use?

### "Sticky Prompt" (Option 1) if:
- ✅ Want simplicity
- ✅ Want to stay in normal stdout mode
- ✅ Don't need to resize dynamically
- ✅ Okay with logs potentially scrolling off screen

### "Alt-Screen" (Option 2) if:
- ✅ Want cleaner separation
- ✅ Need scrollable log history
- ✅ Want to handle terminal resize gracefully
- ✅ Okay with full-screen TUI mode

## 💡 Hybrid Approach (Recommended)

Support **both** via a config flag:

```rust
#[derive(Resource)]
pub enum ReplDisplayMode {
    /// Stdout with sticky prompt at bottom
    StickyPrompt,
    /// Full alt-screen TUI
    AlternateScreen,
}

pub fn render_repl(
    mode: Res<ReplDisplayMode>,
    // ... other params
) {
    match *mode {
        ReplDisplayMode::StickyPrompt => render_sticky_prompt(/*...*/),
        ReplDisplayMode::AlternateScreen => render_alt_screen(/*...*/),
    }
}
```

Let users choose at startup:

```rust
// Configure via CLI or config file
ReplPlugin::new()
    .with_display_mode(ReplDisplayMode::StickyPrompt)
```

## 🎨 Advanced: Smooth Scrolling Effect

For extra polish with the sticky prompt:

```rust
impl PromptRenderer {
    pub fn print_output(&mut self, line: String) {
        self.output_buffer.push_back(line);
        
        // Animate scroll (optional)
        self.scroll_animation();
        
        if self.output_buffer.len() > self.max_output_lines {
            self.output_buffer.pop_front();
        }
    }

    fn scroll_animation(&self) {
        // Quick scroll effect (optional - looks nice!)
        for _ in 0..3 {
            self.redraw_all();
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
```

## 📋 Complete Example: Sticky Prompt System

```rust
// Full working system
fn handle_repl_output_with_sticky_prompt(
    mut renderer: ResMut<PromptRenderer>,
    mut log_reader: MessageReader<LogEvent>,
    terminal_state: Res<TerminalState>,
) {
    // Process any new logs
    for log in log_reader.read() {
        let formatted = format!("[{:5}] {}", log.level, log.message);
        renderer.print_output(formatted);
    }

    // Always render the prompt (maintains cursor position)
    renderer.render_prompt(&terminal_state);
}
```

## 🚀 Yes, This Works Great!

The **sticky prompt pattern** (Option 1) gives you exactly what you described:
- Logs print 1 row above cursor ✅
- Cursor stays at bottom as input line ✅
- Logs scroll by naturally ✅
- Prompt never moves ✅

The implementation is ~150 lines and integrates perfectly with your existing crossterm-based input handling.

---

**My recommendation**: Start with **Option 1 (Sticky Prompt)** for simplicity, then add **Option 2 (Alt-Screen)** as an advanced feature if users want more control over log viewing/scrolling. Both are achievable with the custom crossterm solution!