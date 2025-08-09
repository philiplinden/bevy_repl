use anyhow::Result;
use bevy::prelude::*;

pub mod parser;
pub mod register;

pub use parser::{ParserPlugin, CommandParser, TypedCommandParser, parse_input_buffer_for_commands};
pub use register::{ReplAppExt, register_command_in_repl};

pub type ReplResult<T> = Result<T, clap::error::Error>;

/// Trait for commands that can be registered with the REPL
pub trait ReplCommand: Send + Sync + Clone + Event + Default + 'static {
    /// Returns the clap::Command definition for this command
    fn clap_command() -> clap::Command;

    /// Create the command event from parsed clap argument matches
    fn to_event(_matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(Self::default())
    }

    /// Parse arguments from a string slice
    fn parse(args: &[&str]) -> Result<clap::ArgMatches, clap::Error>
    where
        Self: Sized,
    {
        Self::clap_command().try_get_matches_from(args)
    }
}
