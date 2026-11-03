# Prototype 1a: Raw mode can be entered and always restored

_Archived 2026-11-02 from `prototypes/1a-raw-restore.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out what it takes to leave the terminal usable no matter
  how the app ends. No input handling beyond what exiting needs.
- **Claim:** A Bevy app can turn on raw mode at startup and restore the
  terminal on `AppExit`, on Ctrl+C, and on a panic.
- **Baseline:** If raw mode isn't turned off, the shell is left with no echo
  and, on Unix, broken newlines. While raw mode is on, Ctrl+C is a key press,
  so nothing happens unless the app reads it.
- **Vision:** 8.
- **Proof:** After each of the three exits, the shell echoes typed text and
  newlines behave normally.
- **Watch for:** Raw mode delivers Ctrl+C as a key event instead of a signal,
  on both Unix (no `ISIG`) and Windows (no `ENABLE_PROCESSED_INPUT`). Does
  `TerminalCtrlCHandlerPlugin` ever fire while raw mode is on? Is the `ctrlc`
  dependency needed at all?
- **File:** `proto_1a_raw_restore.rs`
- **Result:**
