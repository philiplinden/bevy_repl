# Logging

This crate supports two complementary logging display strategies. Choose one per app to avoid duplicates.

- Stdout strategy (terminal scrollback)
  - Capture tracing/log events and print to the terminal output area.
  - Works best when not using an alternate screen (plain stdout renderer).
  - Components referenced: `CaptureSubscriber`, `print_log_events_system`.

- In-frame strategy (TUI frame)
  - Captures events into an in-memory buffer and renders logs inside the ratatui frame above the prompt.
  - Works best with an alternate-screen TUI.
  - Components referenced: `LogBufferPlugin`, `LogBuffer`.

Guideline:
- Do not enable both simultaneously.
- Select based on the active renderer (e.g., via `PromptRenderer::strategy()`), and wire only one path into your app.

Notes:
- API surface and specific types may evolve; consult the examples and source code for current wiring.
- Consider adding structured spans around REPL commands to improve trace readability.
