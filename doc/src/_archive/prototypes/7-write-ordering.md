# Prototype 7: Redraws and writes never interleave

_Archived 2026-11-02 from `prototypes/7-write-ordering.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether prompt redraws and other output can tear
  each other, and whether the stdout and stderr locks prevent it.
- **Claim:** Holding the stdout and stderr locks while redrawing the prompt
  keeps prompt escape codes and log output from tearing each other, including
  output from a thread outside Bevy.
- **Baseline:** Each log event is one write on a locked stderr, but a prompt
  redraw is several writes, so another write can land between them.
- **Vision:** 3.
- **Proof:** Log every frame, log a multi-line message, and `println!` from a
  spawned thread, all while typing fast. No escape-code fragments, no text on
  the prompt row.
- **File:** `proto_7_write_ordering.rs`
- **Result:**
