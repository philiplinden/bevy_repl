# Prototype 5: A prompt can be pinned to the bottom row

_Archived 2026-11-02 from `prototypes/5-pinned-prompt.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** First TUI step. Find out whether a scroll region keeps a
  prompt in place while output scrolls.
- **Claim:** A DECSTBM scroll region over every row but the last keeps a
  prompt on the bottom row while `println!` output scrolls above it. The real
  cursor stays in the scroll region, and the prompt's cursor is drawn (reverse
  video).
- **Baseline:** Without a scroll region, all output scrolls the whole screen,
  including anything drawn on the bottom row.
- **Vision:** 2, 3.
- **Proof:** A timer prints a line every 100 ms while typing. The prompt row is
  never overwritten and never moves.
- **File:** `proto_5_pinned_prompt.rs`
- **Result:**
