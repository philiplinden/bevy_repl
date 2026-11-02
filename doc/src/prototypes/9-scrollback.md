# Prototype 9: Output stays after exit

- **Objective:** Find out whether the TUI pieces (scroll region, prompt row)
  can be undone on exit without losing the session's output.
- **Claim:** Resetting the scroll region on exit leaves everything printed
  during the session in the terminal's scrollback.
- **Baseline:** Plain terminal output stays in scrollback. TUIs that use the
  alternate screen lose it on exit.
- **Vision:** 2.
- **Proof:** Quit, scroll up, and read the whole session.
- **File:** `proto_9_scrollback.rs`
- **Result:**
