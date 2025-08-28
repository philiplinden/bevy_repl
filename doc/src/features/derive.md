# Derive
Use the `derive` feature to support clap's derive pattern for REPL commands.
`#[derive(ReplCommand)]` will automatically implement the `ReplCommand` trait
and create an event with the command's arguments and options. Configure the
response by adding an observer for the REPL command like normal.

Enable the `derive` feature to use clap's derive pattern with `#[derive(ReplCommand)]`.

```toml
[dependencies]
bevy_repl = { version = "0.4", features = ["derive"] }
```

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

#[derive(ReplCommand, Parser, Default, Event)]
struct Ping;

fn on_ping(_t: Trigger<Ping>) {
    println!("pong");
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin::default(),
            ReplPlugins,
        ))
        .add_repl_command::<Ping>()
        .add_observer(on_ping)
        .run();
}
```
