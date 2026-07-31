use crate::context::ReplContext;
use crate::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<ClearCommand>();
    app.add_observer(on_clear);
}

#[derive(Event, Clone, Default)]
struct ClearCommand;

impl crate::command::ReplCommand for ClearCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("clear").about("Clears previous outputs from the REPL")
    }
}

fn on_clear(_trigger: On<ClearCommand>, mut terminal: ResMut<ReplContext>) {
    match terminal.clear() {
        Ok(_) => return,
        Err(e) => error!("Failed to clear terminal: {}", e),
    }
}
