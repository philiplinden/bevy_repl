use bevy::prelude::*;

/// Trait for commands that can be registered with the REPL
pub trait ReplCommand: Send + Sync + Clone + Event + 'static {
    /// Returns the clap::Command definition for this command
    fn command() -> clap::Command;
    
    /// Parse the command from command line arguments
    fn parse_from_args(args: &[&str]) -> Result<Self, clap::Error>
    where
        Self: Sized;
}

/// Extension trait for App to register REPL commands using Bevy's observer system
pub trait ReplExt {
    /// Register a command with its observer function using Bevy's Trigger system
    fn repl<C: ReplCommand>(&mut self, observer: impl Fn(Trigger<C>) + Send + Sync + 'static) -> &mut Self;
}

impl ReplExt for App {
    fn repl<C: ReplCommand>(&mut self, observer: impl Fn(Trigger<C>) + Send + Sync + 'static) -> &mut Self {
        // Register the observer using Bevy's observer system
        self.add_observer(observer);
        self
    }
}

/// System that parses terminal input and triggers command observers
pub fn parse_command_input<C: ReplCommand>(
    mut terminal: ResMut<crate::terminal::BevyRatatuiTerminal>,
    mut commands: Commands,
) where
    C: ReplCommand,
{
    // Get the current input line
    let input = terminal.get_current_line().trim();
    
    // Skip empty input
    if input.is_empty() {
        return;
    }
    
    // Split input into arguments
    let args: Vec<&str> = input.split_whitespace().collect();
    
    // Try to parse the input as our command
    match C::parse_from_args(&args) {
        Ok(command) => {
            // Trigger the command using Bevy's observer system
            commands.trigger(command);
            
            // Clear the input line after successful parsing
            terminal.clear_line();
        }
        Err(e) => {
            // Log parsing errors
            terminal.add_log_line(format!("Error: {}", e));
            terminal.clear_line();
        }
    }
}