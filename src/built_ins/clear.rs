use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.repl::<ClearCommand>(on_clear);
}

struct ClearCommand;

impl ReplCommand for ClearCommand {
    fn command() -> clap::Command {
        clap::Command::new("clear")
            .about("Clears previous outputs from the REPL")
    }
}

fn on_clear(trigger: Trigger<ClearCommand>) {
    // TODO: Implement clear command
}