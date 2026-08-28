# Tracing Subscriber Integration

Type: grilling
Status: resolved

## Question

How should Bevy's default logging infrastructure (`bevy_log` / `tracing`) be routed so that log events format cleanly and print into the DECSTBM scroll region without clobbering the active prompt line?

Specifically:
1. How should a custom `tracing_subscriber::Layer` be implemented or attached to Bevy's `LogPlugin`?
2. How does the logger coordinate cursor positioning (e.g. moving to the last scrollable line before writing) to prevent cursor race conditions with the prompt rendering system?

## Answer

### 1. Custom Tracing Layer via `LogPlugin`
Provide a `repl_tracing_layer()` factory function that creates a `tracing_subscriber::Layer` compatible with Bevy's logging setup:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        custom_layer: |_| Some(repl_tracing_layer()),
        ..default()
    }))
    .add_plugins(ReplPlugins)
```

### 2. Scroll-Region Log Positioning & Cursor Restoration
When a log event arrives in `ReplTracingLayer::on_event`:
1. Move the terminal cursor to the bottom-most scrollable row ($H - 1$ in 1-based coordinates, or `rows - 2` in 0-based coordinates).
2. Write the formatted log line with explicit CRLF (`\r\n`). The terminal emulator scrolls rows $1 \dots H-1$ upward, leaving the prompt on row $H$ undisturbed.
3. Reposition the cursor back to the prompt line row $H$ (`rows - 1`) so user typing is not interrupted:

```rust
pub fn write_repl_log(formatted_message: &str) {
    let Ok((_cols, rows)) = crossterm::terminal::size() else { return };
    let scroll_bottom = rows.saturating_sub(2);
    let prompt_row = rows.saturating_sub(1);

    let mut out = std::io::stdout();
    use crossterm::{cursor::MoveTo, queue, style::Print};

    let _ = queue!(
        out,
        MoveTo(0, scroll_bottom),
        Print(formatted_message),
        Print("\r\n"),
        MoveTo(0, prompt_row),
    );
    let _ = out.flush();
}
```

### 3. Graceful Fallback
When the REPL is disabled (`repl.enabled == false`), the tracing layer skips DECSTBM cursor positioning and writes formatted lines directly to stdout with standard CRLF.
