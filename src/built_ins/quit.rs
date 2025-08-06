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
    
    fn parse_from_args(args: &[&str]) -> Result<Self, clap::Error> {
        let matches = Self::command().get_matches_from(args);
        let verbose = matches.get_flag("verbose");
        Ok(QuitCommand { verbose })
    }
}

fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    let command = trigger.event();
    if command.verbose {
        info!("Quitting...");
    }
    exit.send(AppExit::Success);
}
