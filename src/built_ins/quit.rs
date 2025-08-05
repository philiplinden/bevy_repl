use bevy::prelude::*;
use clap::{Command, Arg, ArgAction};
use crate::ReplCommand;

pub fn plugin(app: &mut App) {
    app.repl::<QuitCommand>(on_quit);
}

struct QuitCommand;

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
}

fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    if trigger.verbose {
        info!("Quitting...");
    }
    exit.send(AppExit::Success);
}
