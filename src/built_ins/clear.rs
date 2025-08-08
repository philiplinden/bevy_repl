use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<ClearCommand>();
    app.add_observer(on_clear);
}

#[derive(Event, Clone)]
struct ClearCommand;

impl clap::FromArgMatches for ClearCommand {
    fn from_arg_matches(_matches: &clap::ArgMatches) -> Result<Self, clap::error::Error> {
        Ok(ClearCommand)
    }
    
    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::error::Error> {
        Ok(())
    }
}

impl ReplCommand for ClearCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("clear").about("Clears previous outputs from the REPL")
    }


}

fn on_clear(_trigger: Trigger<ClearCommand>, mut terminal: ResMut<ReplContext>) {
    match terminal.clear() {
        Ok(_) => return,
        Err(e) => error!("Failed to clear terminal: {}", e),
    }
}
