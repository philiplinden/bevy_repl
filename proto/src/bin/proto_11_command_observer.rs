//! Prototype 11: A command can be an event handled by an observer
//!
//! Objective: Find out whether one type can be both the clap command and the
//! Bevy event, for both the derive and builder patterns.
//!
//! Assumption: A clap-derived struct can also be a Bevy event: build it from
//! the parsed matches, trigger it, and handle it with `add_observer`. Same for
//! the builder pattern.
//!
//! Baseline: A clap struct isn't a Bevy event. It needs Bevy's `Event` trait as
//! well.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
