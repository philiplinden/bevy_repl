use bevy::prelude::*;
use crate::{ReplCommand, ReplResult};
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
    fn execute(&self, world: &mut World, _matches: &clap::ArgMatches) -> ReplResult<String> {
        world.send_event(AppExit::Success);
        Ok("Shutting down application...".to_string())
    }
}
