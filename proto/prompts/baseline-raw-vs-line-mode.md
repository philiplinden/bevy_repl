# Prompt: establish the no-TUI baseline (line mode vs raw mode)

Use this to start a session dedicated to prototypes 0, 1a, 1b and 1c.

---

You're helping Philip establish what BevyREPL can do **without a TUI**. The
REPL should work without one if at all possible. Whatever this session finds is
the baseline: later, a TUI (pinned prompt, scroll region) gets tried on top,
and it has to clearly beat this baseline to earn its place.

## How to work with Philip

Philip writes the code and makes the decisions. You guide: explain how
things work, point at sources, propose steps and options, review what he
writes, and explain errors. Don't edit files under `proto/src/` unless he
asks. Propose text for the devlog and let him write or approve it.

## Read first

- `proto/README.md`: where prototypes live and the rules for them.
- `doc/src/prototypes/index.md`: the list of prototypes, with one page each
  in `doc/src/prototypes/` and a log of decisions in
  `doc/src/prototypes/devlog.md`. This session covers **0** (line mode),
  **1a** (raw mode restored on every exit), **1b** (inline editing in raw
  mode) and **1c** (inline editing while logs are busy). Prototype 0's page
  has prep notes checked against the sources. Each has a stub in
  `proto/src/bin/` whose header states its objective, assumption and
  baseline.
- `doc/src/design/index.md` and `doc/src/design/vision.md`: what the REPL is
  for.
- The Windows/Unix pitfall in `doc/src/devlog/03-crossterm-rewrite/2026-11-01.md`.

## What "without a TUI" means here

No scroll region, no pinned prompt, no alternate screen, and no change to
Bevy's `LogPlugin`. Output goes to the terminal the normal way and stays in
scrollback. Two ways to read input:

- **Line mode (no raw mode):** a thread reads whole lines from stdin and
  sends them to Bevy over a channel. The terminal does echo, editing and
  Ctrl+C.
- **Raw mode, inline:** crossterm raw mode, keys read in `PreUpdate` with
  `poll(Duration::ZERO)`, a buffer kept by the app, and the current line
  redrawn in place (carriage return, clear line, prompt, buffer). No
  reserved row. The app owns editing, Ctrl+C and restoring the terminal.

## Same harness for both

Line mode is `proto_0_line_mode.rs`. Raw mode is split across
`proto_1a_raw_restore.rs`, `proto_1b_raw_inline.rs` and
`proto_1c_raw_inline_logs.rs`, one thing each. The scenarios below compare 0
with 1c, which use the same setup:

- `MinimalPlugins` at 60 fps and `LogPlugin::default()`.
- A system that logs `tick {n}` on a timer. Run once with a 100 ms timer
  (busy) and once with no ticks (quiet).
- A system that logs each line it receives: `got: {line}`.
- A way out: `quit` typed as a line, Ctrl+C, and a forced panic.

## Scenarios to run in each mode

1. Type a short command while quiet. Does Enter deliver it intact?
2. Type a command while ticks are logging. What does the screen look like?
   Does the delivered line match what was typed?
3. Backspace across a point where a log line interrupted the typed text.
4. Arrow keys, Home/End.
5. Non-ASCII input (`é`, an emoji).
6. A line longer than the terminal width.
7. Paste a multi-line block.
8. Resize the terminal while typing.
9. Exit three ways: `quit`, Ctrl+C, panic. After each, does the shell echo
   and edit normally? Is the session's output still in scrollback?
10. With `TerminalCtrlCHandlerPlugin` added: does it fire in each mode?

## Record

For each mode and scenario: what happened, on which OS and terminal
(Windows Terminal, conhost, a Linux or macOS terminal). A result isn't final
until it has been run on both Windows and Unix. Expect differences: on
Windows crossterm reads input with `ReadConsoleInput`, which may report keys
even without raw mode; on Unix nothing arrives until Enter.

Then help Philip compare the two modes:

- Which is more usable while logs are busy?
- What does raw mode buy over line mode, and what does it cost (restore
  logic, Ctrl+C handling, newline handling on Unix)?
- Which problems are left for a TUI to solve?

Propose a short summary for the **Result** entries of prototypes 0 and
1a–1c on their pages, and an entry for `doc/src/prototypes/devlog.md`.
Philip decides the wording.
