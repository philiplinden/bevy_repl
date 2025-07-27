use bevy::prelude::*;
use crate::repl::ReplCommand;
use crate::repl::ReplResult;
use clap::{Command, ArgMatches};

/// Quit/Exit command - graceful shutdown
#[derive(Default)]
pub struct QuitCommand;

impl ReplCommand for QuitCommand {
    fn command(&self) -> Command {
        Command::new("quit")
            .about("Gracefully terminate the application")
            .aliases(["exit", "q"])
    }

    // In command.rs QuitCommand
    fn execute(&self, world: &mut World, _matches: &clap::ArgMatches) -> ReplResult<String> {
        if let Some(repl_state) = world.get_resource_mut::<ReplState>() {
            repl_state.request_quit();
            Ok("Shutting down...".to_string())
        } else {
            Err("REPL not running".into())
        }
    }

    fn name(&self) -> &'static str {
        "quit"
    }
}
