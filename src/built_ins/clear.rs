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

    fn execute(trigger: Trigger<Self>) {
        let _command = trigger.event();
        // TODO: Implement clear command
    }
}
