//! Prototype 9: Output stays after exit
//!
//! Objective: Find out whether the TUI pieces (scroll region, prompt row) can
//! be undone on exit without losing the session's output.
//!
//! Assumption: Resetting the scroll region on exit leaves everything printed
//! during the session in the terminal's scrollback.
//!
//! Baseline: Plain terminal output stays in scrollback. TUIs that use the
//! alternate screen lose it on exit.
//!
//! Proof, notes and results:
//! doc/src/prototypes/9-scrollback.md

fn main() {}
