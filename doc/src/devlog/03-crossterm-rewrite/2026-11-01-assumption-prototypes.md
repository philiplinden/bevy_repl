# 2026-11-01 - Prototyping the Design Assumptions

## Why

The [design approach](../../design/index.md) and the
[vision](../../design/vision.md) rest on assumptions about Bevy, crossterm,
clap and terminals that nobody has checked in a running program. Before
changing the crate to match the vision, prove each one by building the
smallest working example of it, as its own program in a `proto` crate.

Rules:

- One prototype proves one thing. No prototype depends on another's code.
- Each prototype says what it claims, which vision item depends on it, and
  what counts as proof.
- Every result records the OS and terminal it ran on (see the Windows/Unix
  pitfall in the [2026-11-01 note](2026-11-01.md)).
- Merging prototypes into one comes after all of them have a result.

## The baseline: no TUI

The REPL should work without a TUI if at all possible. Prototypes 0 and
1a–1c establish what that looks like, and the result is the baseline. A TUI (the
pinned prompt and scroll region in 5–7) gets tried afterward, on top of the
baseline, and it has to clearly improve on it to be worth adding.

"No TUI" means no scroll region, no pinned prompt, no alternate screen, and no
change to `LogPlugin`. Output goes to the terminal the normal way and stays in
scrollback. Two ways to read input get compared under the same setup:

- **Line mode (0):** no raw mode. A thread reads whole lines, and the terminal
  handles echo, editing and Ctrl+C.
- **Raw mode, inline (1a–1c):** the app reads keys, keeps the buffer, and
  redraws the current line in place without reserving a row. Split into three
  prototypes: restoring the terminal (1a), editing a line (1b), and editing
  while logs are busy (1c).

The comparison runs in its own session, started from
`proto/prompts/baseline-raw-vs-line-mode.md`: a shared setup, ten scenarios,
and the questions the comparison has to answer, including which problems are
left for a TUI to solve.

## Where the prototypes live

A `proto/` crate, added as a workspace member and never published:

```text
proto/
  Cargo.toml        # name = "proto", publish = false
  src/bin/
    proto_0_line_mode.rs
    proto_1a_raw_restore.rs
    ...
    minimal_repl.rs  # the merge stage
```

Cargo finds every file in `src/bin/` on its own, so there are no `[[bin]]`
entries to maintain. Run one with
`cargo run -p proto --bin proto_0_line_mode`.

Why a separate crate instead of a `bin/` folder in `bevy_repl`:

- Each prototype can pull in whatever it needs (`crossterm`, `clap`, Bevy's
  windowing) without touching `bevy_repl`'s dependencies.
- Nothing in `proto` is packaged, published or installed with the library.
- As a workspace member it shares the build cache, so Bevy isn't compiled
  twice.

`proto` can depend on `bevy_repl` by path (`bevy_repl = { path = ".." }`).
`bevy_repl` never depends on `proto`, so there's no cycle.

Rules for this setup:

- **One Bevy version.** Both crates take `bevy` from `[workspace.dependencies]`,
  so they share one copy of Bevy and its types match.
- **Prototypes don't use `bevy_repl`.** Its panic hook, Ctrl+C handler and log
  setup would run too, and a result couldn't be traced to one assumption. Only
  the merge stage, or a prototype that compares against the current crate,
  depends on it.
- **Bevy features combine across the workspace.** A build of the whole
  workspace (CI's `clippy --workspace`) compiles one Bevy with the features of
  both crates. Once a prototype turns on windowing, `bevy_repl` could use a
  feature it never asked for and still pass. `cargo check -p bevy_repl` on its
  own catches that.
- **CI checks `proto` too.** Clippy runs with `-D warnings` across the
  workspace, so prototypes are committed warning-free.

## Assumptions to prove

Each entry has:

- **Objective:** what the prototype is meant to explore.
- **Claim:** the assumption being tested.
- **Baseline:** what happens without the technique being tested, as far as
  the sources say. Not yet observed unless a result says so.
- **Proof:** what counts as the claim holding.
- **File:** the program in `proto/src/bin/`.

### 0. A line-mode REPL might be good enough without raw mode

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
- **Watch for:** On Windows, crossterm reads input with `ReadConsoleInput`
  (`crossterm-0.29.0/src/event/source/windows.rs:51`), which may report
  individual keys even without raw mode. On Unix, nothing arrives until Enter.
  Run on both.
- **File:** `proto_0_line_mode.rs`
- **Result:**

### 1a. Raw mode can be entered and always restored

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

### 1b. Inline editing in raw mode, quiet

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

### 1c. Inline editing in raw mode while logs are busy

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

### 2. Keys can be read inside the schedule, headless

- **Objective:** Find out whether reading the terminal fits in Bevy's loop
  without a thread or runner of its own.
- **Claim:** With `MinimalPlugins`, a system in `PreUpdate` can read keys with
  `crossterm::event::poll(Duration::ZERO)` every frame, without a separate
  thread or runner, and without slowing the frame rate.
- **Baseline:** A headless app sees no keys. `MinimalPlugins` doesn't add
  `InputPlugin`, and Bevy's keyboard events come from a window.
- **Vision:** 5, and design statement 2 (no second input system).
- **Proof:** Each key typed is logged once, and the frame rate stays at 60 fps.
- **File:** `proto_2_keys_headless.rs`
- **Result:**

### 3. Keys can be read the same way with a window

- **Objective:** Find out whether the system from 2 behaves the same when
  Bevy has a window.
- **Claim:** The same system works unchanged under `DefaultPlugins` with a
  window, while the terminal (not the window) has focus.
- **Baseline:** With a window, Bevy only gets keys while its window has
  focus. Nothing reads the terminal.
- **Vision:** 5.
- **Proof:** Same as 2. Also log the frame rate while the window is unfocused.
- **Watch for:** Does winit slow or pause updates when its window loses focus?
- **File:** `proto_3_keys_windowed.rs`
- **Result:**

### 4. Typing in the terminal doesn't reach the game

- **Objective:** Find out whether the crate needs any code to keep REPL typing
  away from the game.
- **Claim:** Keys typed into the terminal never show up in Bevy's
  `ButtonInput<KeyCode>` or keyboard messages, so no suppression system is
  needed.
- **Baseline:** The OS sends key presses to the focused window only, so the
  terminal and the game window shouldn't see each other's keys. The current
  crate has a suppression system anyway.
- **Vision:** 4.
- **Proof:** With a window open, type in the terminal while a system logs
  everything Bevy's input sees. Nothing appears. Then focus the window and type
  there: Bevy sees it, the terminal doesn't.
- **File:** `proto_4_input_isolation.rs`
- **Result:**

### 5. A prompt can be pinned to the bottom row

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

### 6. Bevy's unmodified logs land above the prompt

- **Objective:** Find out whether the pinned prompt works with Bevy's own
  logger, untouched.
- **Claim:** With `LogPlugin::default()` left alone, Bevy's log lines (stderr)
  scroll above the prompt exactly like `println!` does in 5.
- **Baseline:** `LogPlugin` writes to stderr at the cursor and knows nothing
  about a prompt (`bevy_log-0.20.1/src/lib.rs:367`).
- **Vision:** 3, 7.
- **Proof:** Same as 5 with `info!` instead of `println!`. `RUST_LOG` still
  filters the output.
- **File:** `proto_6_logs_above_prompt.rs`
- **Result:**

### 7. Redraws and writes never interleave

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

### 8. Newlines behave in raw mode

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

### 9. Output stays after exit

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

### 10. clap can parse a typed line without exiting

- **Objective:** Find out whether clap can be used as a parser for lines,
  rather than for a program's arguments.
- **Claim:** A line split with `shell-words` and parsed with
  `try_get_matches_from` returns matches or an error. Parse errors and `--help`
  come back as text to print, and the process never exits.
- **Baseline:** clap's `get_matches` prints and exits the process on a parse
  error or `--help`.
- **Vision:** 1.
- **Proof:** A plain loop over hard-coded strings (no Bevy, no terminal): a
  valid command, an unknown argument, `--help`, a quoted argument.
- **File:** `proto_10_clap_parse.rs`
- **Result:**

### 11. A command can be an event handled by an observer

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

### 12. The footprint stays small

- **Objective:** Find out what the REPL's dependencies cost on top of Bevy.
- **Claim:** The dependencies above (`crossterm`, `clap`, `shell-words`) are
  all the REPL needs. `tracing-subscriber` isn't.
- **Baseline:** The current crate also pulls in `tracing-subscriber` and
  `ctrlc`.
- **Vision:** 6.
- **Proof:** Compare `cargo tree` and a clean build time of a bare Bevy app
  against one with these dependencies added.
- **File:** none. This is measured with `cargo`, not a program.
- **Result:**

## After the prototypes

Once each one has a result, merge them one at a time into a single
`proto/src/bin/minimal_repl.rs`, rechecking the earlier proofs at each step.
Then the merged version is what the plugin gets measured against.

## Open questions

- Decided: one file per prototype, so each result stays reproducible.
  `minimal_repl.rs` is for the merge stage. They live in the `proto` crate
  (see above), not as examples.
- Raw mode is split into 1a–1c. Whether this numbering and grouping is
  right is still open; revisit once the baseline has results.
- The "Spike: prove option B" plan in the [2026-11-01 note](2026-11-01.md)
  covers 5, 6, 7 and 8 in one example. This list splits it up.
