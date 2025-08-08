use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<HelpCommand>();
    app.add_observer(on_help);
}

#[derive(Event, Clone)]
struct HelpCommand;

impl clap::FromArgMatches for HelpCommand {
    fn from_arg_matches(_matches: &clap::ArgMatches) -> Result<Self, clap::error::Error> {
        Ok(HelpCommand)
    }
    
    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::error::Error> {
        Ok(())
    }
}

impl ReplCommand for HelpCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("help").about("Shows help for the REPL")
    }


}

fn on_help(_t: Trigger<HelpCommand>) {
    info!("Help command not yet implemented.");
}