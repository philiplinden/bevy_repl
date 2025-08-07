use bevy::prelude::*;
use clap::{Command, Arg, ArgAction};
use crate::ReplCommand;

pub fn plugin(app: &mut App) {
    app.repl::<QuitCommand>(on_quit);
}

#[derive(Event)]
struct QuitCommand {
    verbose: bool,
}

impl ReplCommand for QuitCommand {
    fn command() -> clap::Command {
        clap::Command::new("quit")
            .about("Exits the app gracefully")
            .arg(
                clap::Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enables verbose output")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn execute(trigger: Trigger<Self>) {
        let command = trigger.event();
        if command.verbose {
            info!("Quitting...");
        }
        exit.send(AppExit::Success);
    }
}
