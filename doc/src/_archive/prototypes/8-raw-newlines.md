# Prototype 8: Newlines behave in raw mode

_Archived 2026-11-02 from `prototypes/8-raw-newlines.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether normal `\n` output can work in raw mode on
  Unix.
- **Claim:** On Unix, turning output post-processing (`OPOST | ONLCR`) back on
  after entering raw mode makes a bare `\n` return to column 0. On Windows,
  nothing is needed.
- **Baseline:** On Unix, raw mode turns off output processing, so `\n` moves
  down a line without returning to column 0 and output stair-steps. Windows
  raw mode leaves output alone.
- **Vision:** 3.
- **Proof:** Print multi-line output in raw mode with and without the fix.
  Needs a Linux or macOS run, not only Windows.
- **File:** `proto_8_raw_newlines.rs`
- **Result:**
