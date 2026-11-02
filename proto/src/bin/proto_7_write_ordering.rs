//! Prototype 7: Redraws and writes never interleave
//!
//! Objective: Find out whether prompt redraws and other output can tear each
//! other, and whether the stdout and stderr locks prevent it.
//!
//! Assumption: Holding the stdout and stderr locks while redrawing the prompt
//! keeps prompt escape codes and log output from tearing each other, including
//! output from a thread outside Bevy.
//!
//! Baseline: Each log event is one write on a locked stderr, but a prompt
//! redraw is several writes, so another write can land between them.
//!
//! Proof, notes and results:
//! doc/src/prototypes/7-write-ordering.md

fn main() {}
