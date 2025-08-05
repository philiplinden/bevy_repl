use bevy::prelude::*;

/// Trait for commands that can be registered with the REPL
pub trait ReplCommand: Send + Sync + 'static {
    /// Returns the clap::Command definition for this command
    fn command() -> clap::Command;
}

/// Extension trait for App to register REPL commands
pub trait ReplCommandExt {
    /// Register a command with its observer function
    fn add_repl_command<C: ReplCommand, F>(&mut self, observer: F) -> &mut Self
    where
        F: Fn(Trigger<C>) + Send + Sync + 'static;
        
    /// Register a command with its observer function that takes additional parameters
    fn add_repl_command_with<C: ReplCommand, F, Args>(&mut self, observer: F) -> &mut Self
    where
        F: Fn(Trigger<C>, Args) + Send + Sync + 'static;
}

impl ReplCommandExt for App {
    fn add_repl_command<C: ReplCommand, F>(&mut self, observer: F) -> &mut Self
    where
        F: Fn(Trigger<C>) + Send + Sync + 'static,
    {
        // TODO: Implement command registration using Bevy's observer system
        // This will parse commands and trigger the observer when matched
        self.observe(observer)
    }
    
    fn add_repl_command_with<C: ReplCommand, F, Args>(&mut self, observer: F) -> &mut Self
    where
        F: Fn(Trigger<C>, Args) + Send + Sync + 'static,
    {
        // TODO: Implement command registration using Bevy's observer system
        // This will parse commands and trigger the observer when matched
        self.observe(observer)
    }
}