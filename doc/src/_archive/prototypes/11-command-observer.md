# Prototype 11: A command can be an event handled by an observer

_Archived 2026-11-02 from `prototypes/11-command-observer.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether one type can be both the clap command and
  the Bevy event, for both the derive and builder patterns.
- **Claim:** A clap-derived struct can also be a Bevy event: build it from the
  parsed matches, trigger it, and handle it with `add_observer`. Same for the
  builder pattern.
- **Baseline:** A clap struct isn't a Bevy event. It needs Bevy's `Event`
  trait as well.
- **Vision:** 1.
- **Proof:** A hard-coded line triggers an observer that changes a resource.
  No terminal involved.
- **File:** `proto_11_command_observer.rs`
- **Result:**
