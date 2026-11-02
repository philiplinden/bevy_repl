//! Prototype 2: Keys can be read inside the schedule, headless
//!
//! Objective: Find out whether reading the terminal fits in Bevy's loop without
//! a thread or runner of its own.
//!
//! Assumption: With `MinimalPlugins`, a system in `PreUpdate` can read keys
//! with `crossterm::event::poll(Duration::ZERO)` every frame, without a
//! separate thread or runner, and without slowing the frame rate.
//!
//! Baseline: A headless app sees no keys. `MinimalPlugins` doesn't add
//! `InputPlugin`, and Bevy's keyboard events come from a window.
//!
//! Proof and results are in the devlog:
//! doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md

fn main() {}
