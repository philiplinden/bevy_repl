# Crossterm Event Ingestion and Input Suppression

Type: grilling
Status: open
Blocked by: 01

## Question

How should raw crossterm key events be polled in `PreUpdate`, converted into Bevy message events, and kept isolated from normal Bevy game input?

Specifically:
1. How is `crossterm::event::poll(Duration::ZERO)` executed non-blockingly within `ReplSet::Capture`?
2. How are raw `crossterm::event::KeyEvent` instances translated into atomic `ReplBufferEvent` actions?
3. How is `ButtonInput<KeyCode>` and `KeyboardInput` suppressed in `PreUpdate`/`Post` so that keystrokes typed into the REPL never trigger in-game systems?
4. How is the toggle key (e.g. `` ` ``) detected and filtered before it enters the text buffer?
