# Prototype 0: A line-mode REPL might be good enough without raw mode

- **Objective:** Find out how usable a REPL is when the terminal does all the
  input work. This is one half of the no-TUI baseline.
- **Claim:** With raw mode off, a small thread reads whole lines from stdin
  and passes them to Bevy over a channel. The terminal handles echo, editing
  and Ctrl+C, so there is no terminal state to restore.
- **Baseline:** A headless Bevy app reads nothing from stdin. Whatever is
  typed is echoed by the terminal and left unread, and the shell probably
  receives it after the app exits.
- **Vision:** 3, 8. If this is good enough, prototypes 1a, 1b, 1c and 8
  aren't needed, and the pinned prompt (5–7) becomes optional.
- **Proof:** Log a line every 100 ms while typing a command. Enter delivers the
  full line to a Bevy system even when a log line split the visible text.
  Ctrl+C exits through `TerminalCtrlCHandlerPlugin`. Decide whether the mixed
  output is acceptable.
- **Watch for:** Backspace, arrow keys and history after a log line split the
  typed text. Whether a half-typed line at exit ends up in the shell. Run on
  both Windows and Unix.
- **File:** `proto_0_line_mode.rs`
- **Result:**

## Prep notes

Checked against the sources before building (Rust nightly std, Bevy 0.20.1).
None of it has been run yet.

- **No crossterm needed.** Line mode reads with `std::io::stdin().read_line`.
  The only dependency is Bevy, already in `proto/Cargo.toml`. (An earlier
  version of this entry worried about crossterm's Windows key events; that
  only matters if crossterm reads the input, and here it doesn't.)
- **Windows stdin is a console read.** When stdin is a console, std reads it
  with `ReadConsoleW` in cooked mode (`std/src/sys/stdio/windows.rs`,
  `read_u16s`). Text comes in as UTF-16 and is converted, so non-ASCII input
  should work.
- **Ctrl+C doesn't break the read on Windows.** `ReadConsoleW` returns early
  with `ERROR_OPERATION_ABORTED` on Ctrl+C, and std retries the read instead of
  returning. So `read_line` keeps waiting, and the Ctrl+C reaches Bevy's
  handler as normal. (I had guessed the opposite while planning.)
- **Ctrl+Z is end of input on Windows.** std wakes the read on Ctrl+Z and
  drops it, so Ctrl+Z on an empty line makes `read_line` return `Ok(0)`. On
  Unix it's Ctrl+D. The reader thread should stop on `Ok(0)`.
- **Ctrl+C on Unix** is a signal, because the terminal isn't in raw mode.
  `TerminalCtrlCHandlerPlugin` installs a handler with the `ctrlc` crate and
  sends `AppExit` from a system in `Update`. `MinimalPlugins` doesn't include
  it, so it's added by hand (`bevy::app::TerminalCtrlCHandlerPlugin`).
- **The channel can't be a plain resource.** In Bevy 0.20 a `Resource` is a
  `Component`, which must be `Send + Sync`. `std::sync::mpsc::Receiver` isn't
  `Sync`. Either wrap it in a `Mutex`, or store it with
  `App::insert_non_send` and read it with `NonSend`. A non-send resource pins
  the system that reads it to the main thread.
- **Exiting with the thread still blocked is fine.** When `main` returns, the
  process ends, even if the reader thread is still inside `read_line`.
- **The terminal edits the line, not the app.** Expected but not checked:
  - Unix canonical mode gives Backspace, Ctrl+U (erase line), Ctrl+W (erase
    word), and Ctrl+R to reprint the pending line after output split it.
  - The Windows console gives arrow keys, Home/End and command history
    (Up arrow, F7) for free.
- **Piped input works too.** In line mode, `echo quit | cargo run -p proto
  --bin proto_0_line_mode` should behave like typing `quit`. Raw mode can't
  do that, since it needs a real terminal. Useful for scripted tests.

## Build steps

Each step runs on its own before the next one is added.

1. **Vanilla headless app.** `MinimalPlugins` at 60 fps, `LogPlugin`, and
   `TerminalCtrlCHandlerPlugin`, as on the
   [vanilla page](../design/vanilla.md). A system logs `tick {n}` every
   100 ms. Check that Ctrl+C exits.
2. **Reader thread.** Before building the app, make a channel and spawn a
   thread that loops on `read_line` and sends each line. Stop on `Ok(0)` (end
   of input) or an error.
3. **Lines into Bevy.** Store the receiver (behind a `Mutex`, or as a
   non-send resource) and drain it with `try_recv` in a `PreUpdate` system
   that logs `got: {line}`. A line reading `quit` sends `AppExit`.
4. **Run the scenarios** from `proto/prompts/baseline-raw-vs-line-mode.md`,
   busy and quiet, then pipe input in. Record the OS and terminal.
