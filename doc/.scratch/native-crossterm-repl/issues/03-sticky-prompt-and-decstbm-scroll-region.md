# Sticky Prompt and DECSTBM Scroll Region

Type: prototype
Status: resolved

## Question

What is the exact ANSI escape sequence flow and system ordering required to maintain a sticky prompt line at the bottom of the terminal while allowing output to scroll cleanly above it?

Specifically:
1. How and when is the DECSTBM scroll region (`\x1B[1;{bottom}r`) initialized and reset?
2. How should the prompt line (prefix symbol + buffer text + cursor position) be redrawn per frame without screen flickering?
3. How are explicit CRLF (`\r\n`) and column positioning enforced across all terminal write paths?

## Answer

### 1. DECSTBM Scroll Region Lifecycle
- **Setup & Sizing**:
  When the REPL is enabled on a terminal of height $H$ rows with a 1-line prompt, emit `\x1B[1;{H - 1}r` to establish the scroll margin for rows $1$ through $H-1$. Row $H$ is physically excluded from terminal scrolling.
- **Teardown & Reset**:
  When the REPL is disabled, during `ReplTerminal::restore()` or on application exit, emit `\x1B[r` to restore the full-window scroll region.

### 2. Flicker-Free Prompt Rendering in `ReplSet::Render`
To eliminate flickering, render the prompt only when the REPL buffer, cursor, prompt configuration, or terminal dimensions change (`Changed<Repl>`, `Changed<ReplPromptConfig>`, or resize events):

```rust
pub fn render_prompt(
    repl: Res<Repl>,
    prompt: Res<ReplPrompt>,
    mut last_rendered: Local<String>,
) {
    if !repl.enabled {
        return;
    }

    let Ok((_cols, rows)) = crossterm::terminal::size() else { return };
    let prompt_row = rows - 1; // 0-based bottom row

    let rendered_line = format!("{}{}", prompt.symbol.as_deref().unwrap_or("> "), repl.buffer);

    if *last_rendered != rendered_line || repl.is_changed() {
        let mut out = std::io::stdout();
        use crossterm::{cursor::MoveTo, style::Print, terminal::{Clear, ClearType}, queue};
        
        let cursor_x = (prompt.symbol.as_deref().unwrap_or("> ").len() + repl.cursor_pos) as u16;
        
        let _ = queue!(
            out,
            MoveTo(0, prompt_row),
            Clear(ClearType::CurrentLine),
            Print(&rendered_line),
            MoveTo(cursor_x, prompt_row), // Restore cursor to typing position
        );
        let _ = out.flush();
        *last_rendered = rendered_line;
    }
}
```

### 3. Dynamic Terminal Resizing
Upon receiving a terminal resize event (`crossterm::event::Event::Resize(cols, rows)`):
1. Query new dimensions (`terminal::size()`).
2. Re-send `\x1B[1;{rows - 1}r` to adjust the scroll boundary.
3. Invalidate `last_rendered` to redraw the prompt at the new bottom row.

### 4. Future Enhancements & Alt-Screen Fallback
- **Dynamic Multi-line Prompt & Status Gutters**: The scroll region calculation (`H - N`) is naturally extensible to reserve $N$ lines for wrapped prompts or debug status lines (FPS, memory stats) in a future polish pass.
- **Alt-Screen Fallback**: If DECSTBM exhibits emulator-specific quirks, the rendering strategy can pivot to `EnterAlternateScreen` as an alternative display adapter without affecting the core input or parser subsystems.
