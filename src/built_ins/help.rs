use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.repl::<HelpCommand>(on_help);
}

#[derive(Event)]
struct HelpCommand;

impl ReplCommand for HelpCommand {
    fn command() -> clap::Command {
        clap::Command::new("help")
            .about("Shows help for the REPL")
    }

    fn execute(trigger: Trigger<Self>) {
        let _command = trigger.event();
        // TODO: Implement help command
    }
}
