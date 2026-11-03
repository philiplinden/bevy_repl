# Prototype 4: Typing in the terminal doesn't reach the game

_Archived 2026-11-02 from `prototypes/4-input-isolation.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether the crate needs any code to keep REPL typing
  away from the game.
- **Claim:** Keys typed into the terminal never show up in Bevy's
  `ButtonInput<KeyCode>` or keyboard messages, so no suppression system is
  needed.
- **Baseline:** The OS sends key presses to the focused window only, so the
  terminal and the game window shouldn't see each other's keys. The current
  crate has a suppression system anyway.
- **Vision:** 4.
- **Proof:** With a window open, type in the terminal while a system logs
  everything Bevy's input sees. Nothing appears. Then focus the window and type
  there: Bevy sees it, the terminal doesn't.
- **File:** `proto_4_input_isolation.rs`
- **Result:**
