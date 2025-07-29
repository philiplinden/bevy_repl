#![doc = include_str ! ("../README.md")]

pub mod repl;
pub mod registry;
pub mod built_ins;
pub mod error;
pub mod config;
pub mod command;

pub mod prelude {
    pub use crate::{repl::{ReplCommand, ReplResult}, config::ReplConfig, registry::ReplCommandRegistration, built_ins::{HelpCommand, QuitCommand, TreeCommand}};
    #[cfg(feature = "diagnostics")]
    pub use crate::built_ins::SysInfoCommand;
}

use crate::{
    config::ReplConfig,
    repl::{ReplState, Repl, ReplSet},
    registry::ReplCommandRegistry,
};
use bevy::prelude::*;

/// The main REPL plugin
pub struct ReplPlugin {
    config: Option<ReplConfig>,
}

impl ReplPlugin {
    /// Create a new REPL plugin with default configuration
    pub fn new() -> Self {
        Self { config: None }
    }

    /// Create a new REPL plugin with custom configuration
    pub fn with_config(config: ReplConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Default for ReplPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Insert the configuration resource
        let config = self.config.clone().unwrap_or_default();
        app.insert_resource(config)
            .insert_resource(ReplCommandRegistry::default())
            .insert_resource(ReplState::default())
            .add_event::<ReplCommandEntered>()
            .add_event::<PrintReplLine>()
            // Add the REPL resource - this starts the rustyline thread
            .insert_resource(Repl::with_config(config.clone()))
            // Add systems to handle REPL input/output
            .add_systems(Update, (
                repl_input_system,
                repl_output_system,
            ).in_set(ReplSet::ReplUI))
            .add_systems(Update, (
                command_execution_system,
            ).in_set(ReplSet::Commands))
            .add_systems(Update, (
                quit_check_system,
            ).in_set(ReplSet::PostCommands));
    }
}

/// Run condition which does not run any command systems if no command was entered
fn commands_entered(commands: EventReader<ReplCommandEntered>) -> bool {
    !commands.is_empty()
}

/// System that reads input from the rustyline thread and processes commands
/// This runs every frame but only processes input when available
fn repl_input_system(
    mut repl: ResMut<Repl>,
    mut registry: ResMut<ReplCommandRegistry>,
    mut world: &mut World,
    mut command_events: EventWriter<ReplCommandEntered>,
) {
    // Check for quit request first
    if repl.should_quit() {
        return;
    }
    
    // Try to receive input from the rustyline thread
    while let Some(input) = repl.try_recv_input() {
        // Parse and execute the command
        match registry.parse_and_execute(&input, &mut world) {
            Ok(output) => {
                // Send output back to be printed
                repl.send_output(output);
                command_events.send(ReplCommandEntered);
            }
            Err(e) => {
                // Send error message to user
                repl.send_output(format!("Error: {}", e));
                command_events.send(ReplCommandEntered);
            }
        }
    }
}

/// System that handles REPL output
/// This ensures output is printed in a thread-safe way
fn repl_output_system(
    mut print_events: EventReader<PrintReplLine>,
    repl: Res<Repl>,
) {
    for event in print_events.read() {
        repl.send_output(event.0.clone());
    }
}

/// System that executes commands
/// This is where custom command logic would go
fn command_execution_system(
    mut commands: EventReader<ReplCommandEntered>,
) {
    // Process any command events
    for _ in commands.read() {
        // Command execution is handled in repl_input_system
        // This system can be used for additional command processing if needed
    }
}

/// System that checks if the REPL should quit
/// This allows the Bevy app to exit gracefully when the user types 'quit'
fn quit_check_system(
    repl: Res<Repl>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
) {
    if repl.should_quit() {
        app_exit.send(bevy::app::AppExit::Success);
    }
}

/// Event that is sent when a command is entered
#[derive(Event)]
pub struct ReplCommandEntered;

/// Event that can be sent to print a line to the REPL
#[derive(Event)]
pub struct PrintReplLine(pub String);
