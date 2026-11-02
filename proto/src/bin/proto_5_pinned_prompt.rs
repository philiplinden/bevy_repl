//! Prototype 5: A prompt can be pinned to the bottom row
//!
//! Objective: First TUI step. Find out whether a scroll region keeps a prompt
//! in place while output scrolls.
//!
//! Assumption: A DECSTBM scroll region over every row but the last keeps a
//! prompt on the bottom row while `println!` output scrolls above it. The real
//! cursor stays in the scroll region, and the prompt's cursor is drawn (reverse
//! video).
//!
//! Baseline: Without a scroll region, all output scrolls the whole screen,
//! including anything drawn on the bottom row.
//!
//! Proof, notes and results:
//! doc/src/prototypes/5-pinned-prompt.md

fn main() {}
