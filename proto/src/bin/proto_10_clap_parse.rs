//! Prototype 10: clap can parse a typed line without exiting
//!
//! Objective: Find out whether clap can be used as a parser for lines, rather
//! than for a program's arguments.
//!
//! Assumption: A line split with `shell-words` and parsed with
//! `try_get_matches_from` returns matches or an error. Parse errors and
//! `--help` come back as text to print, and the process never exits.
//!
//! Baseline: clap's `get_matches` prints and exits the process on a parse error
//! or `--help`.
//!
//! Proof, notes and results:
//! doc/src/prototypes/10-clap-parse.md

fn main() {}
