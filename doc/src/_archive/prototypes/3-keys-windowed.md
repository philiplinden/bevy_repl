# Prototype 3: Keys can be read the same way with a window

_Archived 2026-11-02 from `prototypes/3-keys-windowed.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether the system from 2 behaves the same when
  Bevy has a window.
- **Claim:** The same system works unchanged under `DefaultPlugins` with a
  window, while the terminal (not the window) has focus.
- **Baseline:** With a window, Bevy only gets keys while its window has
  focus. Nothing reads the terminal.
- **Vision:** 5.
- **Proof:** Same as 2. Also log the frame rate while the window is unfocused.
- **Watch for:** Does winit slow or pause updates when its window loses focus?
- **File:** `proto_3_keys_windowed.rs`
- **Result:**
