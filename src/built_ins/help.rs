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
    
    fn parse_from_args(args: &[&str]) -> Result<Self, clap::Error> {
        let _matches = Self::command().get_matches_from(args);
        Ok(HelpCommand)
    }
}

fn on_help(trigger: Trigger<HelpCommand>) {
    let _command = trigger.event();
    // TODO: Implement help command
}