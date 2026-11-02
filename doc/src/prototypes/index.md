# Prototypes

## Why

The [design approach](../design/index.md) and the
[vision](../design/vision.md) rest on assumptions about Bevy, crossterm,
clap and terminals that nobody has checked in a running program. Before
changing the crate to match the vision, prove each one by building the
smallest working example of it, as its own program in a `proto` crate.

Rules:

- One prototype proves one thing. No prototype depends on another's code.
- Each prototype says what it claims, which vision item depends on it, and
  what counts as proof.
- Every result records the OS and terminal it ran on (see the Windows/Unix
  pitfall in the [2026-11-01 note](../devlog/03-crossterm-rewrite/2026-11-01.md)).
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

## The prototypes

Each entry has:

- **Objective:** what the prototype is meant to explore.
- **Claim:** the assumption being tested.
- **Baseline:** what happens without the technique being tested, as far as
  the sources say. Not yet observed unless a result says so.
- **Proof:** what counts as the claim holding.
- **File:** the program in `proto/src/bin/`.
- **Result:** what happened, with the OS and terminal it ran on.

- [0. A line-mode REPL might be good enough without raw mode](0-line-mode.md)
- [1a. Raw mode can be entered and always restored](1a-raw-restore.md)
- [1b. Inline editing in raw mode, quiet](1b-raw-inline.md)
- [1c. Inline editing in raw mode while logs are busy](1c-raw-inline-logs.md)
- [2. Keys can be read inside the schedule, headless](2-keys-headless.md)
- [3. Keys can be read the same way with a window](3-keys-windowed.md)
- [4. Typing in the terminal doesn't reach the game](4-input-isolation.md)
- [5. A prompt can be pinned to the bottom row](5-pinned-prompt.md)
- [6. Bevy's unmodified logs land above the prompt](6-logs-above-prompt.md)
- [7. Redraws and writes never interleave](7-write-ordering.md)
- [8. Newlines behave in raw mode](8-raw-newlines.md)
- [9. Output stays after exit](9-scrollback.md)
- [10. clap can parse a typed line without exiting](10-clap-parse.md)
- [11. A command can be an event handled by an observer](11-command-observer.md)
- [12. The footprint stays small](12-footprint.md)

The [devlog](devlog.md) records how this plan came together and what each
session found.

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
- The "Spike: prove option B" plan in the
  [2026-11-01 note](../devlog/03-crossterm-rewrite/2026-11-01.md) covers 5, 6, 7 and 8 in one example. This list splits it up.
