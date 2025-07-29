use bevy::prelude::*;
use std::collections::BTreeMap;

/// Configuration for the REPL plugin
#[derive(Resource, Clone)]
pub struct ReplConfig {
    /// Whether to enable the REPL
    pub enabled: bool,

    /// The prompt string to display
    pub prompt: String,

    /// The key to toggle the REPL. If None, the REPL is always enabled.
    pub toggle_key: Option<KeyCode>,

    /// Registered console commands
    pub commands: BTreeMap<&'static str, clap::Command>,

    #[cfg(feature = "custom-history-file")]
    /// Custom history file path. If None, uses rustyline's default (~/.rustyline_history)
    /// This allows users to have separate history files for different Bevy apps
    pub history_file: Option<String>,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            enabled: true,
            toggle_key: None,
            commands: BTreeMap::new(),
            #[cfg(feature = "custom-history-file")]
            history_file: None,
        }
    }
}

impl ReplConfig {
    /// Create a new REPL configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom prompt string
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }

    #[cfg(feature = "custom-history-file")]
    /// Set a custom history file path
    /// This allows different Bevy apps to have separate command histories
    /// Example: .with_history_file(".my_game_history")
    pub fn with_history_file(mut self, history_file: impl Into<String>) -> Self {
        self.history_file = Some(history_file.into());
        self
    }

    /// Disable the REPL
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
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
