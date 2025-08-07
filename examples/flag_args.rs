use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

// Define a simple command struct
#[derive(Debug, Clone, Event)]
struct GreetCommand {
    name: String,
    shout: bool,
}

// Implement ReplCommand trait
impl ReplCommand for GreetCommand {
    fn command() -> clap::Command {
        clap::Command::new("greet")
            .about("Greet someone")
            .arg(
                clap::Arg::new("name")
                    .help("Name of the person to greet")
                    .required(true),
            )
            .arg(
                clap::Arg::new("shout")
                    .short('s')
                    .long("shout")
                    .help("Shout the greeting")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn from_matches(matches: clap::ArgMatches) -> Self {
        let name = matches.get_one::<String>("name").unwrap().clone();
        let shout = matches.get_flag("shout");
        GreetCommand { name, shout }
    }
}

// System that handles the command with access to Bevy ECS
fn on_greet(trigger: Trigger<GreetCommand>) {
    let command = trigger.event();
    let greeting = if command.shout {
        format!("HELLO, {}!", command.name.to_uppercase())
    } else {
        format!("Hello, {}!", command.name)
    };
    println!("{}", greeting);
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
        .add_repl_command::<GreetCommand>(on_greet)
        .run();
}
