# proto

Small throwaway programs that test the assumptions BevyREPL's design rests on,
one at a time, before the crate is changed to match the
[vision](../doc/src/design/vision.md). The list of assumptions, what counts as
proof, and the results live in the devlog:
[Prototyping the Design Assumptions](../doc/src/devlog/03-crossterm-rewrite/2026-11-01-assumption-prototypes.md).

This crate is a workspace member and is never published.

## Layout

Each file in `src/bin/` is one prototype, named after its number in the
devlog (`proto_0_line_mode.rs`, `proto_1a_raw_restore.rs`, ...). Cargo finds
them on its own. Each file starts with what the prototype explores (objective),
the assumption it tests, and what happens without it (baseline). Prototype 12
(footprint) is measured with `cargo` and has no file. `minimal_repl.rs` is
reserved for merging the prototypes together once they all have a result.

```sh
cargo run -p proto --bin proto_0_line_mode
```

## Rules

- **One prototype proves one thing.** No prototype uses another's code.
- **Prototypes don't depend on `bevy_repl`.** Its panic hook, Ctrl+C handler
  and log setup would run too. Only the merge stage, or a prototype that
  compares against the current crate, depends on it.
- **Bevy comes from `[workspace.dependencies]`**, so `proto` and `bevy_repl`
  share one copy. Add the Bevy features and other crates a prototype needs
  here, in `proto/Cargo.toml`.
- **Bevy features combine across the workspace.** After turning on a feature
  here, run `cargo check -p bevy_repl` on its own to make sure the library
  doesn't depend on it by accident.
- **Commit prototypes warning-free.** CI runs clippy with `-D warnings` across
  the workspace.
- **Record every result with the OS and terminal it ran on.** Windows and Unix
  terminals behave differently.
