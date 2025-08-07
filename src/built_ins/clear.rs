use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<ClearCommand>();
    app.add_observer(on_clear);
}

#[derive(Event, Clone)]
struct ClearCommand;

impl ReplCommand for ClearCommand {
    fn command() -> clap::Command {
        clap::Command::new("clear").about("Clears previous outputs from the REPL")
    }

    fn from_matches(_matches: clap::ArgMatches) -> Self {
        ClearCommand
    }
}

fn on_clear(_trigger: Trigger<ClearCommand>, mut terminal: ResMut<ReplContext>) {
    match terminal.clear() {
        Ok(_) => return,
        Err(e) => error!("Failed to clear terminal: {}", e),
    }
}
