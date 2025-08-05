use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.repl::<HelpCommand>(on_help);
}

struct HelpCommand;

impl ReplCommand for HelpCommand {
    fn command() -> clap::Command {
        clap::Command::new("help")
            .about("Shows help for the REPL")
    }
}

fn on_help(trigger: Trigger<HelpCommand>) {
    // TODO: Implement help command
}