use bevy::prelude::*;
use crate::prelude::*;
use crate::repl_println;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<HelpCommand>();
    app.add_observer(on_help);
}

#[derive(Event, Clone, Default)]
struct HelpCommand;

impl crate::command::ReplCommand for HelpCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("help").about("Shows help for the REPL")
    }
}

fn on_help(_t: Trigger<HelpCommand>) {
    repl_println!("not implemented, sorry");
}
