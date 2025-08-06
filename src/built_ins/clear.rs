use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.repl::<ClearCommand>(on_clear);
}

#[derive(Event)]
struct ClearCommand;

impl ReplCommand for ClearCommand {
    fn command() -> clap::Command {
        clap::Command::new("clear")
            .about("Clears previous outputs from the REPL")
    }
    
    fn parse_from_args(args: &[&str]) -> Result<Self, clap::Error> {
        let _matches = Self::command().get_matches_from(args);
        Ok(ClearCommand)
    }
}

fn on_clear(trigger: Trigger<ClearCommand>) {
    let _command = trigger.event();
    // TODO: Implement clear command
}