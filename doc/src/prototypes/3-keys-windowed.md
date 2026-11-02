# Prototype 3: Keys can be read the same way with a window

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
