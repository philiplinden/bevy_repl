# bevy_repl logging strategies

This note explains the two ways logs are routed and displayed alongside the REPL prompt.

## Strategies

- **Stdout strategy (CaptureSubscriber + print system)**
  - Tracing -> events -> printed via `repl_println!` to the terminal scrollback.
  - The prompt stays at the bottom; new logs scroll above it.
  - Use with `renderer::stdout::StdoutRenderer`.
  - Wiring in `PromptPlugin.build()`:
    - `CaptureSubscriberPlugin::default()`
    - `print_log_events_system` (prints lines using `repl_print.rs::repl_print()` which respects the reserved bottom region)

- **In-frame buffer strategy (LogBufferPlugin)**
  - Tracing -> events -> drained into an in-memory `LogBuffer` resource.
  - The renderer reads `ctx.logs` and draws them inside the ratatui frame (alternate screen) above the prompt block.
  - Use with `renderer::alt_screen::AltScreenRenderer`.
  - Wiring in `PromptPlugin.build()`:
    - `LogBufferPlugin::default()` (also installs capture + drains events into the buffer)

## Key differences

- **Where logs end up**
  - Stdout strategy: outside ratatui, in the terminal scrollback.
  - In-frame buffer: inside the ratatui frame on the alternate screen.

- **Who displays logs**
  - Stdout strategy: `print_log_events_system` uses `repl_println!` to write lines, managing cursor/scroll region so they appear above the prompt.
  - In-frame buffer: the renderer draws `ctx.logs` within the frame.

- **Terminal mode**
  - Stdout: normal screen (no alternate screen required).
  - In-frame: alternate screen; logs + prompt are drawn entirely by ratatui.

## Don’t mix both displays

Avoid enabling both stdout printing and in-frame rendering simultaneously to prevent duplicate logs. Pick one per renderer via `PromptRenderer::strategy()`.

## Practical tip for alt-screen

Keep a bottom scroll region reserved even in alt-screen so stray stdout from third-party code cannot overwrite the prompt block; this ensures unexpected prints scroll above the prompt area.
