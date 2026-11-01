use crate::prelude::*;
use crate::repl_println;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<HelpCommand>();
    app.add_observer(on_help);
}

#[derive(Event, Clone)]
pub struct HelpCommand;

impl crate::command::ReplCommand for HelpCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("help")
            .visible_alias("h")
            .about("Shows available REPL commands")
    }

    fn to_event(_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self)
    }
}

fn on_help(_trigger: On<HelpCommand>, repl: Res<Repl>) {
    repl_println!("Available commands:");
    for name in repl.commands.keys() {
        repl_println!("  {}", name);
    }
}
