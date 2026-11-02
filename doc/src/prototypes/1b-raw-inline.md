# Prototype 1b: Inline editing in raw mode, quiet

- **Objective:** Find out what the app has to do itself to edit a line once
  the terminal stops helping. No logging, so editing is tested on its own.
- **Claim:** Keys read in raw mode go into a buffer, and redrawing the current
  line in place (carriage return, clear the line, prompt, buffer) is enough to
  edit a line without reserving a row. Enter delivers the line.
- **Baseline:** In raw mode the terminal echoes nothing and edits nothing.
  Typed keys are invisible until the app draws them.
- **Vision:** 1, 4.
- **Proof:** Type, Backspace, arrow keys, Home/End, non-ASCII input (`é`, an
  emoji), and a line wider than the terminal. What's on screen always matches
  the buffer, and Enter delivers it.
- **File:** `proto_1b_raw_inline.rs`
- **Result:**
