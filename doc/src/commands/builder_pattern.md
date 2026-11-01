# Builder pattern

Use clap's [builder pattern] to describe the command and its arguments or
options by implementing `ReplCommand`:

- `clap_command()` returns the `clap::Command` that describes the command.
- `to_event()` converts the parsed `clap::ArgMatches` into the command struct.

Then add the command to the app with `.add_repl_command::<SayCommand>()` and
handle it with an observer. The command struct is also the event, so the
observer reads its fields as the parsed arguments and options.

**Example**:
[builder.rs](https://github.com/philiplinden/bevy_repl/blob/main/examples/builder.rs)

[builder pattern]: https://docs.rs/clap/latest/clap/_tutorial/index.html

```rust,ignore
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct SayCommand {
    message: String,
}

impl ReplCommand for SayCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(SayCommand {
            message: matches.get_one::<String>("message").unwrap().clone(),
        })
    }
}

fn on_say(trigger: On<SayCommand>) {
    repl_println!("{}", trigger.event().message);
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
