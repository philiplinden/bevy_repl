# Prototype 2: Keys can be read inside the schedule, headless

_Archived 2026-11-02 from `prototypes/2-keys-headless.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether reading the terminal fits in Bevy's loop
  without a thread or runner of its own.
- **Claim:** With `MinimalPlugins`, a system in `PreUpdate` can read keys with
  `crossterm::event::poll(Duration::ZERO)` every frame, without a separate
  thread or runner, and without slowing the frame rate.
- **Baseline:** A headless app sees no keys. `MinimalPlugins` doesn't add
  `InputPlugin`, and Bevy's keyboard events come from a window.
- **Vision:** 5, and design statement 2 (no second input system).
- **Proof:** Each key typed is logged once, and the frame rate stays at 60 fps.
- **File:** `proto_2_keys_headless.rs`
- **Result:**
