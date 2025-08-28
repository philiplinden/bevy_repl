# Derive pattern (requires `derive` feature)

Enable the `derive` feature in your `Cargo.toml` to use the derive pattern.

**Example**:
[derive.rs](https://github.com/philiplinden/bevy_repl/blob/main/examples/derive.rs)

```toml
[dependencies]
bevy_repl = { version = "0.3.1", features = ["derive"] }
```

Then derive the `ReplCommand` trait on your command struct along with clap's
`Parser` trait. Add the command and its observer to the app as usual.

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

#[derive(ReplCommand, Parser, Default, Event)]
struct CommandWithoutArgs;

fn on_command_without_args(_trigger: Trigger<CommandWithoutArgs>) {
    println!("You triggered a command without args");
}

#[derive(ReplCommand, Parser, Event, Default)]
#[clap(about = "A command with args")]
struct CommandWithArgs {
    #[clap(short, long)]
    arg1: String,
    #[clap(short, long)]
    arg2: String,
}

fn on_command_with_args(trigger: Trigger<CommandWithArgs>) {
    println!("You triggered a command with args: {} {}", trigger.arg1, trigger.arg2);
}

fn main() {
    App::new()
        .add_plugins((
            // Run headless in the terminal
            MinimalPlugins.set(
                bevy::app::ScheduleRunnerPlugin::run_loop(
                    Duration::from_secs_f32(1. / 60.)
                )
            ),
            // Bevy input plugin is required to detect keyboard inputs
            bevy::input::InputPlugin::default(),
            // Default REPL stack (ratatui, prompt, and built-in commands)
            ReplPlugins,
        ))
        .add_repl_command::<CommandWithoutArgs>()
        .add_observer(on_command_without_args)
        .add_repl_command::<CommandWithArgs>()
        .add_observer(on_command_with_args)
        .run();
}
```
