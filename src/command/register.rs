use super::{CommandParser, ReplCommand, TypedCommandParser};
use crate::repl::Repl;
use bevy::prelude::*;

/// Extension trait for App to add REPL commands
pub trait ReplAppExt {
    /// Add a REPL command with its observer function
    fn add_repl_command<C: ReplCommand>(&mut self) -> &mut Self;
}

impl ReplAppExt for App {
    fn add_repl_command<C: ReplCommand>(&mut self) -> &mut Self {
        self.add_systems(Startup, register_command_in_repl::<C>);
        self
    }
}

// System to register commands in the REPL
pub fn register_command_in_repl<C: ReplCommand>(mut repl: ResMut<Repl>) {
    let cmd = C::clap_command();
    let primary = cmd.get_name().to_string();
    // Insert primary name
    repl.commands.insert(
        primary,
        Box::new(TypedCommandParser::<C>::new()) as Box<dyn CommandParser>,
    );
    // Insert all aliases (visible/invisible)
    for alias in cmd.get_all_aliases() {
        repl.commands.insert(
            alias.to_string(),
            Box::new(TypedCommandParser::<C>::new()) as Box<dyn CommandParser>,
        );
    }
}
