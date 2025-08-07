use anyhow::Result;
use bevy::prelude::*;
use bevy_ratatui::event::InputSet;

use crate::{prompt::ReplSubmitEvent, repl::Repl};

pub struct ParserPlugin;

impl Plugin for ParserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            parse_input_buffer_for_commands.in_set(InputSet::EmitBevy),
        );
    }
}

/// Extension trait for App to add REPL commands
pub trait ReplAppExt {
    /// Add a REPL command with its observer function
    fn add_repl_command<C: ReplCommand>(&mut self) -> &mut Self;
}

impl ReplAppExt for App {
    fn add_repl_command<C: ReplCommand>(&mut self) -> &mut Self {
        // Add the command event type
        self.add_event::<C>();

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
            Err(clap_error) => {
                // Print the Clap error message with preserved formatting
                eprintln!("{}", clap_error);
                true // Return true to indicate we handled this command (even though it failed)
            }
        }
    }
}

// System to register commands in the REPL
pub fn register_command_in_repl<C: ReplCommand>(mut repl: ResMut<Repl>) {
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
        Self::command().try_get_matches_from(args)
    }
}

/// System that parses terminal input and triggers command observers
pub fn parse_input_buffer_for_commands(
    mut submitted_text: EventReader<ReplSubmitEvent>,
    mut bevy_commands: Commands,
    repl: Res<Repl>,
) {
    for event in submitted_text.read() {
        let input = event.0.clone();
        // Skip empty input
        if input.is_empty() {
            continue;
        }
        // Try each registered command parser
        let mut command_handled = false;
        for parser in repl.commands.values() {
            if parser.parse_and_trigger(&input, &mut bevy_commands) {
                command_handled = true;
                break; // Command was handled (either successfully or with error)
            }
        }

        // No command matched
        if !command_handled {
            error!("Unknown command '{}'", input);
        }
    }
}
