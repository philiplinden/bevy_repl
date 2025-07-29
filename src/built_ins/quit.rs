use bevy::prelude::*;
use crate::{ReplCommand, ReplResult};
use clap::Command;

/// Quit/Exit command - graceful shutdown
#[derive(Default, Clone)]
pub struct QuitCommand;

impl ReplCommand for QuitCommand {
    fn command(&self) -> Command {
        Command::new("quit")
            .about("Gracefully terminate the application")
            .aliases(["exit", "q"])
    }
    fn execute(&self, commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        commands.send_event(AppExit::Success);
        Ok("Shutting down application...".to_string())
    }
}
