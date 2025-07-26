//! A simple REPL plugin for Bevy applications
//! 
//! This plugin provides a command-line interface that runs in a separate thread
//! and allows you to execute commands in your Bevy application.

pub mod repl;
pub mod commands;

use bevy::prelude::*;
use std::sync::mpsc::Sender;

/// Configuration for the REPL plugin
#[derive(Resource, Clone)]
pub struct ReplConfiguration {
    /// The prompt string to display
    pub prompt: String,
    /// Whether to enable the REPL on startup
    pub enabled: bool,
}

impl Default for ReplConfiguration {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            enabled: true,
        }
    }
}

/// The main REPL plugin for Bevy applications
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplConfiguration::default())
            .add_systems(Startup, setup_repl_systems)
            .add_systems(Startup, register_default_commands_system.after(setup_repl_systems))
            .add_systems(Update, repl::command_executor);
    }
}

/// System to setup REPL resources and start input thread
fn setup_repl_systems(
    config: Res<ReplConfiguration>,
    mut commands: Commands,
) {
    if config.enabled {
        let (tx, rx) = std::sync::mpsc::channel();
        commands.insert_resource(repl::CommandQueue(std::sync::Mutex::new(rx)));
        commands.insert_resource(commands::CommandRegistry::default());
        repl::start_repl_input(tx);
    }
}

/// System to register default commands after resources are available
fn register_default_commands_system(registry: Res<commands::CommandRegistry>) {
    commands::register_default_commands(registry);
}

/// Setup function to initialize the REPL with custom configuration
pub fn setup_repl_with_config(app: &mut App, config: ReplConfiguration) {
    app.insert_resource(config);
    app.add_plugins(ReplPlugin);
}

/// Setup function to initialize the REPL with default configuration
pub fn setup_repl(app: &mut App) {
    setup_repl_with_config(app, ReplConfiguration::default());
}

// Re-export commonly used items
pub use commands::{CommandHandler, CommandRegistry};