# bevy_repl_derive

Derive macros for the [`bevy_repl`](../README.md) crate.

This crate provides the `#[derive(ReplCommand)]` macro for automatically implementing REPL commands.

## Usage

This crate is typically not used directly. Instead, enable the `derive` feature on the main `bevy_repl` crate:

```toml
[dependencies]
bevy_repl = { version = "0.1.0", features = ["derive"] }
```

Then use the derive macro:

```rust
use bevy_repl::prelude::*;

#[derive(ReplCommand)]
#[command(name = "hello", about = "Say hello to the world")]
struct HelloCommand;
```

## Features

- Automatic implementation of `Default`, `Clone`, and `ReplCommand` traits
- Support for command attributes: `name`, `about`, `aliases`
- Clean, declarative syntax
- Optional override of default implementations

See the main [`bevy_repl`](../README.md) documentation for complete usage examples. 
