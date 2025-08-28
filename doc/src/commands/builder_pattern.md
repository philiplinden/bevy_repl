# Builder pattern

Use clap's [builder pattern] to describe the command and its arguments or
options. Then add the command to the app with
`.add_repl_command<YourReplCommand>()`. The REPL fires an event when the command
is parsed from the prompt. The REPL command struct is also the event. When it is
read by an observer or event reader, you can treat the command as an ordinary
event where its fields are the parsed arguments and options.

Make an observer for the command with `.add_observer(your_observer)`. The
observer is a one-shot system that receives a trigger event with the command's
arguments and options. As a system, it is executed in the `PostUpdate` schedule
and has full access to the Bevy ECS.

[builder pattern]: https://docs.rs/clap/latest/clap/_tutorial/index.html#tutorial-for-the-builder-api

```rust
use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    let mut app = App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
        ));

    app.add_plugins((
        ReplPlugin,
        ReplDefaultCommandsPlugin,
    ))
    .add_repl_command::<SayCommand>()
    .add_observer(on_say);

    app.run();
}

struct SayCommand {
    message: String,
}

impl ReplCommand for SayCommand {
    fn command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .short('m')
                    .long("message")
                    .help("Message to say")
                    .required(true)
                    .takes_value(true)
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(SayCommand {
            message: matches.get_all::<String>("message").unwrap().join(" "),
        })
    }
}

fn on_say(trigger: Trigger<SayCommand>) {
    println!("{}", trigger.message);
}
```
