//! Prototype 1b: Inline editing in raw mode, quiet
//!
//! Objective: Find out what the app has to do itself to edit a line once the
//! terminal stops helping. No logging, so editing is tested on its own.
//!
//! Assumption: Keys read in raw mode go into a buffer, and redrawing the
//! current line in place (carriage return, clear the line, prompt, buffer) is
//! enough to edit a line without reserving a row. Enter delivers the line.
//!
//! Baseline: In raw mode the terminal echoes nothing and edits nothing. Typed
//! keys are invisible until the app draws them.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
