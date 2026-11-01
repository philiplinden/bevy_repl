# Derive pattern

The derive pattern requires the `derive` feature, which is on by default.

Derive clap's `Parser` and the REPL's `ReplCommand` on your command struct,
along with `Event`. Then add the command and its observer to the app as usual.

**Example**:
[derive.rs](https://github.com/philiplinden/bevy_repl/blob/main/examples/derive.rs)

```rust,ignore
use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

#[derive(Parser, ReplCommand, Debug, Clone, Event, Default)]
#[command(name = "say", about = "Say something")]
struct SayCommand {
    #[arg(help = "Message to say")]
    message: String,
    #[arg(short, long, default_value = "1", help = "Number of times to repeat")]
    repeat: usize,
}

fn on_say(trigger: On<SayCommand>) {
    let command = trigger.event();
    for _ in 0..command.repeat {
        repl_println!("{}", command.message);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            )),
            ReplPlugins,
        ))
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .run();
}
```
