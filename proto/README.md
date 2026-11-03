# proto

Small throwaway programs that answer BevyREPL's open design questions before
the crate is changed. The questions come out of the design work that
restarted on 2026-11-02 (see that day's devlog in the book).

The first set of prototypes was written for a design that has since been
dropped. Its plan is archived in `doc/src/_archive/prototypes/`, and its code
is on the `prototypes` branch.

This crate is a workspace member and is never published.

## Layout

Each file in `src/bin/` is one prototype, and Cargo finds them on its own.
Run one with:

```sh
cargo run -p proto --bin <name>
```

## Practical notes

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
