# Prototypes Devlog

_Archived 2026-11-02 from `prototypes/devlog.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

Working notes for the prototype work, newest last. What each prototype found
goes on its own page; this log records the decisions and how the plan changed.

## 2026-11-01: Setting up

**Why prototypes.** The design rests on assumptions about Bevy, crossterm,
clap and terminals that nobody has checked in a running program. The list of
assumptions started as a devlog note in the crossterm-rewrite era and moved
here once it needed one page per prototype.

**Line mode joined the list.** I asked whether crossterm removes the need for
raw mode. It doesn't: raw mode is a terminal setting, and crossterm only
switches it on and off. Without raw mode the terminal buffers input until
Enter, echoes it wherever the cursor is, and handles Backspace and Ctrl+C
itself. That made a line-mode REPL worth testing as prototype 0: nothing to
restore on exit, at the cost of typed text and logs mixing.

**The no-TUI baseline.** The REPL should work without a TUI if at all
possible. Prototypes 0 and 1a–1c set the baseline, and a TUI (the pinned
prompt in 5–7) has to clearly improve on it. The comparison has its own
session prompt in `proto/prompts/baseline-raw-vs-line-mode.md`.

**Raw mode became three prototypes:** restoring the terminal (1a), editing a
line in place (1b), and editing while logs are busy (1c). Not fully settled on
this grouping; revisit once the baseline has results.

**The `proto` crate.** Prototypes live in their own workspace member, never
published, one program per file in `proto/src/bin/`. Each file opens with the
prototype's objective, assumption and baseline. Bevy comes from
`[workspace.dependencies]` so both crates share one copy, and CI's package
step skips `proto`. Started as a top-level `bin/` folder, which Cargo doesn't
find without `[[bin]]` entries.

**Prepared prototype 0.** Checked std, Bevy and terminal behavior against the
sources before writing any code. The notes are on the
[prototype 0 page](0-line-mode.md#prep-notes). The surprise: on Windows,
Ctrl+C doesn't interrupt `read_line`, because std retries the console read.
