//! Prototype 0: A line-mode REPL might be good enough without raw mode
//!
//! Objective: Find out how usable a REPL is when the terminal does all the
//! input work. This is one half of the no-TUI baseline.
//!
//! Assumption: With raw mode off, a small thread reads whole lines from stdin
//! and passes them to Bevy over a channel. The terminal handles echo, editing
//! and Ctrl+C, so there is no terminal state to restore.
//!
//! Baseline: A headless Bevy app reads nothing from stdin. Whatever is typed is
//! echoed by the terminal and left unread, and the shell probably receives it
//! after the app exits.
//!
//! Proof, notes and results:
//! doc/src/prototypes/0-line-mode.md

fn main() {}
