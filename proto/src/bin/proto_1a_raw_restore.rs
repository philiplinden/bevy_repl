//! Prototype 1a: Raw mode can be entered and always restored
//!
//! Objective: Find out what it takes to leave the terminal usable no matter how
//! the app ends. No input handling beyond what exiting needs.
//!
//! Assumption: A Bevy app can turn on raw mode at startup and restore the
//! terminal on `AppExit`, on Ctrl+C, and on a panic.
//!
//! Baseline: If raw mode isn't turned off, the shell is left with no echo and,
//! on Unix, broken newlines. While raw mode is on, Ctrl+C is a key press, so
//! nothing happens unless the app reads it.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
