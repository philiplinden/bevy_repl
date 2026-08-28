# Unified Headless Toggle and Schedule Gating

Type: grilling
Status: resolved

## Question

How should the terminal input capture and toggle mechanics be structured so that headless and windowed applications can seamlessly enable, disable, and toggle the REPL at runtime without requiring duplicate Bevy `ButtonInput` listeners or getting blocked by schedule run conditions?

Specifically:
1. Why does the dual-input path (Bevy `ButtonInput` when disabled vs Crossterm when enabled) fail in headless apps, and how does a single always-on `capture_terminal_input` system resolve it?
2. Which specific schedule sets should be gated by `run_if(repl_is_enabled)` (e.g. `Print` and `InputSuppression`) vs left un-gated (e.g. `Capture` and `Lifecycle`)?
3. How does this simplify user-facing examples by eliminating boilerplate toggle-listener systems?

## Answer

### 1. Always-On Non-Blocking Ingestion in `PreUpdate`
In headless apps, Bevy has no Winit window, so `ButtonInput<KeyCode>` never sees cooked terminal stdin. To resolve this without blocking the frame tick:
- `capture_terminal_input` runs every frame in `PreUpdate` (`ReplSet::Capture`) with non-blocking `crossterm::event::poll(Duration::ZERO)`.
- **When Disabled (`!repl.enabled`)**: It checks `keymap.check_lifecycle(&key_event)`. If F3 is pressed, it emits `ReplLifecycleEvent::Enable`. It ignores all other characters (pass-through).
- **When Enabled (`repl.enabled`)**: It handles editing keys, Ctrl+C, and emits command submissions.

### 2. Selective Schedule Gating
- Gated with `run_if(repl_is_enabled)`: `ReplSet::Print` (prompt redraw) and `suppress_game_keyboard_input`.
- Un-gated (always active): `ReplSet::Capture` (for lifecycle keys) and `handle_repl_lifecycle`.

### 3. Zero-Boilerplate for Users
Applications and examples do not need custom `toggle_when_disabled` systems; the REPL opens and closes automatically on `F3` across both headless and windowed modes.
