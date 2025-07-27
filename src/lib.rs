#![doc = include_str ! ("../README.md")]

pub mod repl;
pub mod registry;
pub mod built_ins;
pub mod error;
pub mod config;
pub mod command;
#[cfg(feature = "history")]
pub mod history;
#[cfg(feature = "suggestions")]
pub mod suggestions;

pub mod prelude {
    pub use crate::{repl::{ReplCommand, ReplResult}, config::ReplConfig, registry::ReplCommandRegistration, built_ins::{HelpCommand, QuitCommand, TreeCommand}};
    #[cfg(feature = "diagnostics")]
    pub use crate::built_ins::SysInfoCommand;
}

use crate::{
    config::ReplConfig,
    repl::ReplState,
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
        .add_event::<ReplCommandEntered>()
        .add_event::<PrintReplLine>();
    }
}

/// Run condition which does not run any command systems if no command was entered
fn commands_entered(commands: EventReader<ReplCommandEntered>) -> bool {
    !commands.is_empty()
}
