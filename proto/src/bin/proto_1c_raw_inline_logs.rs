//! Prototype 1c: Inline editing in raw mode while logs are busy
//!
//! Objective: Find out how raw-mode inline editing holds up against Bevy's
//! logs. This is the half of the no-TUI baseline that gets compared with line
//! mode (0).
//!
//! Assumption: Redrawing the line after each frame's logs keeps the typed line
//! readable, even though log lines can land in the middle of it.
//!
//! Baseline: Bevy's logger writes to stderr wherever the cursor is. The cursor
//! sits at the end of the typed line, so log text lands right after it.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
