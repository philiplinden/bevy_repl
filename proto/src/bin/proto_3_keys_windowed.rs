//! Prototype 3: Keys can be read the same way with a window
//!
//! Objective: Find out whether the system from 2 behaves the same when Bevy has
//! a window.
//!
//! Assumption: The same system works unchanged under `DefaultPlugins` with a
//! window, while the terminal (not the window) has focus.
//!
//! Baseline: With a window, Bevy only gets keys while its window has focus.
//! Nothing reads the terminal.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
