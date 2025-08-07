use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<HelpCommand>();
    app.add_observer(on_help);
}

#[derive(Event, Clone)]
struct HelpCommand;

impl ReplCommand for HelpCommand {
    fn command() -> clap::Command {
        clap::Command::new("help").about("Shows help for the REPL")
    }

    fn from_matches(_matches: clap::ArgMatches) -> Self {
        HelpCommand
    }
}

fn on_help(_t: Trigger<HelpCommand>) {
    info!("Help command not yet implemented.");
}