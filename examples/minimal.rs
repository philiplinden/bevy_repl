use bevy::prelude::*;
use bevy_repl::prelude::*;
use bevy_ratatui::RatatuiPlugins;

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    let mut app =App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
        ));

    app.add_plugins((
        RatatuiPlugins::default(),
        ReplPlugin,
        ReplDefaultCommandsPlugin,
    ))
    .repl::<SayCommand>(on_say);

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
}

fn on_say(trigger: Trigger<SayCommand>) {
    println!("{}", trigger.message);
}

