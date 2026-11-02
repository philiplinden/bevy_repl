# Prototype 1c: Inline editing in raw mode while logs are busy

- **Objective:** Find out how raw-mode inline editing holds up against Bevy's
  logs. This is the half of the no-TUI baseline that gets compared with
  line mode (0).
- **Claim:** Redrawing the line after each frame's logs keeps the typed line
  readable, even though log lines can land in the middle of it.
- **Baseline:** Bevy's logger writes to stderr wherever the cursor is. The
  cursor sits at the end of the typed line, so log text lands right after it.
- **Vision:** 3.
- **Proof:** Same as 1b, with a log line every 100 ms. Decide whether the
  result is acceptable, and compare it with 0.
- **File:** `proto_1c_raw_inline_logs.rs`
- **Result:**
