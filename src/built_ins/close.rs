use bevy::prelude::*;
use crate::{ReplCommand, ReplResult, ReplDisableEvent};
use clap::{Command, ArgMatches};

/// Close command - closes the REPL but does not exit the application
#[derive(Default, Clone)]
pub struct CloseReplCommand;

impl ReplCommand for CloseReplCommand {
    fn command(&self) -> Command {
        Command::new("close")
            .about("Close the REPL but do not exit the application")
            .aliases(["quit", "exit", "q"])
    }

    fn execute(&self, commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        commands.trigger(ReplDisableEvent);
        Ok("REPL disabled. Application will continue running.".to_string())
    }
}
