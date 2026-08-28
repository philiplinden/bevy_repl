# Crossterm Event Ingestion and Input Suppression

Type: grilling
Status: resolved

## Question

How should raw crossterm key events be polled in `PreUpdate`, converted into Bevy message events, and kept isolated from normal Bevy game input?

Specifically:
1. How is `crossterm::event::poll(Duration::ZERO)` executed non-blockingly within `ReplSet::Capture`?
2. How are raw `crossterm::event::KeyEvent` instances translated into atomic `ReplBufferEvent` actions?
3. How is `ButtonInput<KeyCode>` and `KeyboardInput` suppressed in `PreUpdate`/`Post` so that keystrokes typed into the REPL never trigger in-game systems?
4. How is the toggle key (e.g. `` ` ``) detected and filtered before it enters the text buffer?

## Answer

### 1. Non-Blocking In-Schedule Event Polling
Execute `crossterm::event::poll(Duration::ZERO)` inside a non-blocking while loop in `PreUpdate` (`ReplSet::Capture`). Filter for `KeyEventKind::Press | KeyEventKind::Repeat` to ensure cross-platform consistency (filtering out Windows release events).

```rust
pub fn poll_terminal_events(
    mut key_events: EventWriter<TerminalKeyEvent>,
    repl: Res<Repl>,
) {
    if !repl.enabled {
        return;
    }

    while crossterm::event::poll(std::time::Duration::ZERO).unwrap_or(false) {
        if let Ok(crossterm::event::Event::Key(key_event)) = crossterm::event::read() {
            if matches!(
                key_event.kind,
                crossterm::event::KeyEventKind::Press | crossterm::event::KeyEventKind::Repeat
            ) {
                key_events.send(TerminalKeyEvent(key_event));
            }
        }
    }
}
```

### 2. Configurable Keymap & Convenience Plugin
Keybindings and editing actions are **not hard-coded** into the ingestion loop. Instead, a `PromptKeymap` resource maps `KeyEvent` patterns to `ReplBufferEvent` actions, populated by default via a convenience plugin (`ReplDefaultKeymapPlugin`):

- **Standard Navigation & Editing**:
  - `KeyCode::Char(c)` (no ctrl) $\to$ `ReplBufferEvent::Insert(c)`
  - `KeyCode::Backspace` $\to$ `ReplBufferEvent::Backspace`
  - `KeyCode::Delete` $\to$ `ReplBufferEvent::Delete`
  - `KeyCode::Left` / `Right` $\to$ `ReplBufferEvent::MoveLeft` / `MoveRight`
  - `KeyCode::Home` / `End` $\to$ `ReplBufferEvent::JumpToStart` / `JumpToEnd`
- **Submission & Multiline Input**:
  - `KeyCode::Enter` (no modifier) $\to$ `ReplBufferEvent::Submit`
  - `KeyCode::Enter` + `KeyModifiers::SHIFT` $\to$ `ReplBufferEvent::Insert('\n')` (multiline editing)
- **Unix-Style Line Control**:
  - `Ctrl + C` $\to$ `ReplBufferEvent::Clear` (clears active buffer line)
  - `Ctrl + L` $\to$ `ReplBufferEvent::ClearScreen` (triggers screen refresh)
  - `Ctrl + U` $\to$ `ReplBufferEvent::ClearToStart` (Unix line kill from cursor to start)

Users can customize bindings by mutating `PromptKeymap` or disabling the default keymap plugin entirely.

### 3. State-Based Input Suppression
When `repl.enabled` is active, the terminal has exclusive input focus. A system in `PreUpdate` resets Bevy's game input resources to ensure keystrokes typed in the terminal do not trigger game actions:

```rust
pub fn suppress_game_keyboard_input(
    mut key_events: ResMut<Events<bevy::input::keyboard::KeyboardInput>>,
    mut keyboard_input: ResMut<ButtonInput<bevy::input::keyboard::KeyCode>>,
    repl: Res<Repl>,
) {
    if repl.enabled {
        key_events.clear();
        keyboard_input.reset_all();
    }
}
```

### 4. Toggle Key Interception
Before passing keys to the buffer, compare the event code against the configured toggle key (default: `KeyCode::Char('`')`). If matched:
- Emit `ReplLifecycleEvent::Disable` to exit the REPL.
- `continue` without writing the toggle character into the text buffer.
