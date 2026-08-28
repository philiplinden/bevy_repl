use super::ReplCommand;
use crate::repl::{Repl, ReplSubmitEvent};
use crate::repl_println;
use bevy::prelude::*;

pub struct ParserPlugin;

impl Plugin for ParserPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(parse_input_buffer_for_commands);
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
    fn parse_and_trigger(&self, input: &str, bevy_commands: &mut Commands) -> bool {
        // Tokenize the input like a shell (handles quotes/escapes)
        let argv = match shell_words::split(input) {
            Ok(v) => v,
            Err(_) => {
                // Not a valid shell-like input; let other parsers try
                return false;
            }
        };

        // Empty input shouldn't be handled here
        if argv.is_empty() {
            return false;
        }

        // Try parsing with clap; argv already includes the command/alias at [0]
        let cmd = C::clap_command();
        match cmd.try_get_matches_from(&argv) {
            Ok(matches) => {
                match C::to_event(&matches) {
                    Ok(event) => bevy_commands.trigger(event),
                    Err(clap_error) => {
                        for line in format!("{}", clap_error).lines() {
                            repl_println!("{}", line);
                        }
                    }
                }
                true
            }
            Err(clap_error) => {
                // If this looks like an unrelated command token, let others try
                use clap::error::ErrorKind;
                match clap_error.kind() {
                    ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                        // Print help/version text via REPL so it appears above the prompt
                        for line in format!("{}", clap_error).lines() {
                            repl_println!("{}", line);
                        }
                        true
                    }
                    ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand => false,
                    _ => {
                        // Print the Clap error message with preserved formatting
                        for line in format!("{}", clap_error).lines() {
                            repl_println!("{}", line);
                        }
                        true
                    }
                }
            }
        }
    }
}

/// Observer that parses submitted terminal input and triggers command observers
pub fn parse_input_buffer_for_commands(
    trigger: Trigger<ReplSubmitEvent>,
    mut bevy_commands: Commands,
    repl: Res<Repl>,
) {
    let input = &trigger.event().0;
    // Skip empty input
    if input.is_empty() {
        return;
    }

    // Tokenize input and dispatch by the first token (command name or alias)
    let argv = match shell_words::split(input) {
        Ok(v) => v,
        Err(_) => {
            repl_println!("Invalid input: {}", input);
            return;
        }
    };
    if argv.is_empty() {
        return;
    }

    let key = &argv[0];
    if let Some(parser) = repl.commands.get(key) {
        let _ = parser.parse_and_trigger(input, &mut bevy_commands);
    } else {
        repl_println!(
            "Unknown command '{}'. Type 'help' to see available commands.",
            key
        );
    }
}
