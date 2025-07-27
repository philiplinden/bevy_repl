use bevy::prelude::*;
use std::collections::BTreeMap;

/// Configuration for the REPL plugin
#[derive(Resource, Clone)]
pub struct ReplConfig {
    /// Whether to enable the REPL
    pub enabled: bool,

    /// The prompt string to display
    pub prompt: String,

    /// Registered console commands
    pub commands: BTreeMap<&'static str, clap::Command>,

    #[cfg(feature = "history")]
    /// Number of commands to store in history
    pub history_size: usize,

    #[cfg(feature = "suggestions")]
    /// Number of suggested commands to show
    pub num_suggestions: usize,

    #[cfg(feature = "suggestions")]
    /// Custom completion sequences,
    /// for example [vec!["custom", "foo"]], will complete `custom foo` when typing `custom`
    pub arg_completions: Vec<Vec<String>>,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            enabled: true,
            commands: BTreeMap::new(),
            #[cfg(feature = "history")]
            history_size: 100,
            #[cfg(feature = "suggestions")]
            num_suggestions: 5,
            #[cfg(feature = "suggestions")]
            arg_completions: Vec::new(),
        }
    }
}



fn enable_repl(mut config: ResMut<ReplConfig>) {
    info!("Starting Bevy REPL...");
    info!("Type 'help' for available commands, 'quit' to exit.");
    config.enabled = true;
}

fn disable_repl(mut config: ResMut<ReplConfig>) {
    info!("Stopping Bevy REPL...");
    config.enabled = false;
}

fn toggle_repl(config: ResMut<ReplConfig>) {
    match config.enabled {
        true => disable_repl(config),
        false => enable_repl(config),
    }
}
