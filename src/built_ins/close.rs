use bevy::prelude::*;
use crate::{Repl, disable_repl, ReplCommand, ReplResult};
use clap::{Command, ArgMatches};

/// Close command - closes the REPL but does not exit the application
#[derive(Default)]
pub struct CloseCommand;

impl ReplCommand for CloseCommand {
    fn command(&self) -> Command {
        Command::new("close")
            .about("Close the REPL but do not exit the application")
            .aliases(["quit", "exit", "q"])
    }

    fn execute(&self, world: &mut World, _matches: &clap::ArgMatches) -> ReplResult<String> {
        if let Some(mut repl) = world.get_resource_mut::<Repl>() {
            disable_repl(&mut repl);
            Ok("REPL disabled. Application will continue running.".to_string())
        } else {
            Err("REPL not running".into())
        }
    }
}
