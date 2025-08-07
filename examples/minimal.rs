use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

// Define a simple command struct
#[derive(Debug, Clone, Event)]
struct SayCommand {
    message: String,
    repeat: usize,
}

// Implement ReplCommand trait with builder pattern
impl ReplCommand for SayCommand {
    fn command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
            .arg(
                clap::Arg::new("repeat")
                    .short('r')
                    .long("repeat")
                    .help("Number of times to repeat")
                    .default_value("1"),
            )
    }

    fn from_matches(matches: clap::ArgMatches) -> Self {
        let message = matches.get_one::<String>("message").unwrap().clone();
        let repeat = matches.get_one::<String>("repeat")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        
        SayCommand { message, repeat }
    }
}

// System that handles the command with access to Bevy ECS
fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    // Print the main message
    info!("Saying: {}", command.message);
    
    // Print repeated messages
    for i in 0..command.repeat {
        info!("{}: {}", i + 1, command.message);
    }
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            bevy::input::InputPlugin::default(),
            bevy::log::LogPlugin::default(),
            ReplPlugins,
        ))
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .run();
}
