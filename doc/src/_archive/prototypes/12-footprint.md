# Prototype 12: The footprint stays small

_Archived 2026-11-02 from `prototypes/12-footprint.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out what the REPL's dependencies cost on top of Bevy.
- **Claim:** The dependencies above (`crossterm`, `clap`, `shell-words`) are
  all the REPL needs. `tracing-subscriber` isn't.
- **Baseline:** The current crate also pulls in `tracing-subscriber` and
  `ctrlc`.
- **Vision:** 6.
- **Proof:** Compare `cargo tree` and a clean build time of a bare Bevy app
  against one with these dependencies added.
- **File:** none. This is measured with `cargo`, not a program.
- **Result:**
