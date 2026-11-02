//! Prototype 6: Bevy's unmodified logs land above the prompt
//!
//! Objective: Find out whether the pinned prompt works with Bevy's own logger,
//! untouched.
//!
//! Assumption: With `LogPlugin::default()` left alone, Bevy's log lines
//! (stderr) scroll above the prompt exactly like `println!` does in 5.
//!
//! Baseline: `LogPlugin` writes to stderr at the cursor and knows nothing about
//! a prompt (`bevy_log-0.20.1/src/lib.rs:367`).
//!
//! Proof, notes and results:
//! doc/src/prototypes/6-logs-above-prompt.md

fn main() {}
