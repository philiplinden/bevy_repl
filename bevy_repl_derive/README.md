# bevy_repl_derive

Derive macros for [`bevy_repl`](https://crates.io/crates/bevy_repl).

You don't need to depend on this crate directly. `bevy_repl` re-exports the
`ReplCommand` derive when its `derive` feature is enabled (it is by default):

```rust,ignore
use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

#[derive(Parser, ReplCommand, Clone, Event, Default)]
#[command(name = "say", about = "Say something")]
struct SayCommand {
    message: String,
}
```

The generated code calls `clap` directly, so your crate needs `clap` as a
dependency with its `derive` feature enabled.

See the [`bevy_repl` repository](https://github.com/philiplinden/bevy_repl) and
the [BevyREPL Book](https://philiplinden.github.io/bevy_repl/) for more.

## License

Dual-licensed under either the [MIT License](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
