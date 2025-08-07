use bevy::prelude::*;
use anyhow::Result;

use crate::{prompt::ParseReplBufferEvent, repl::Repl};


pub struct ParserPlugin;

impl Plugin for ParserPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(parse_input_buffer_for_commands);
    }
}

/// Extension trait for App to add REPL commands
pub trait ReplAppExt {
    /// Add a REPL command with its observer function
    fn add_repl_command<C: ReplCommand>(
        &mut self,
        observer: impl Fn(Trigger<C>) + Send + Sync + 'static,
    ) -> &mut Self;
}

impl ReplAppExt for App {
    fn add_repl_command<C: ReplCommand>(
        &mut self,
        observer: impl Fn(Trigger<C>) + Send + Sync + 'static,
    ) -> &mut Self {
        // Add the command event type
        self.add_event::<C>();
        
        // Add observer for the command
        self.add_observer(observer);
        
        // Register command in the REPL
        self.add_systems(Startup, register_command_in_repl::<C>);
        
        self
    }
}

pub trait CommandParser: Send + Sync {
    fn parse_and_trigger(&self, input: &str, commands: &mut Commands) -> bool;
}

pub struct TypedCommandParser<C: ReplCommand> {
    _phantom: std::marker::PhantomData<C>,
}

impl<C: ReplCommand> TypedCommandParser<C> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: ReplCommand> CommandParser for TypedCommandParser<C> {
    fn parse_and_trigger(&self, input: &str, commands: &mut Commands) -> bool {
        // Split input into command name and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        
        let command_name = parts[0];
        if command_name != C::command().get_name() {
            return false;
        }
        
        // Parse arguments using clap
        match C::parse_from_args(&parts) {
            Ok(matches) => {
                // Create the command instance from matches
                let command = C::from_matches(matches);
                commands.trigger(command);
                true
            }
            Err(_) => false,
        }
    }
}

// System to register commands in the REPL
pub fn register_command_in_repl<C: ReplCommand>(
    mut repl: ResMut<Repl>,
) {
    let command_name = C::command().get_name().to_string();
    let parser = Box::new(TypedCommandParser::<C>::new()) as Box<dyn CommandParser>;
    repl.commands.insert(command_name, parser);
}

pub type ReplResult<T> = Result<T, anyhow::Error>;

/// Trait for commands that can be registered with the REPL
pub trait ReplCommand: Send + Sync + Clone + Event + 'static {
    /// Returns the clap::Command definition for this command
    fn command() -> clap::Command;

    /// Create the command from parsed clap matches
    fn from_matches(matches: clap::ArgMatches) -> Self;

    /// Parse the command from command line arguments
    fn parse_from_args(args: &[&str]) -> Result<clap::ArgMatches, clap::Error>
    where
        Self: Sized,
    {
        let matches = Self::command().get_matches_from(args);
        Ok(matches)
    }
}

/// System that parses terminal input and triggers command observers
pub fn parse_input_buffer_for_commands(
    input: Trigger<ParseReplBufferEvent>,
    repl: Res<Repl>,
    mut bevy_commands: Commands,
) {
    // Skip empty input
    if input.buffer.is_empty() {
        return;
    }
    
    // Try each registered command parser
    for parser in repl.commands.values() {
        if parser.parse_and_trigger(&input.buffer, &mut bevy_commands) {
            return; // Command was successfully parsed and triggered
        }
    }
    
    // No command matched
    error!("Unknown command: {}", input.buffer);
}
