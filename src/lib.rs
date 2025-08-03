#![doc = include_str ! ("../README.md")]

pub mod built_ins;
pub mod terminal;

// Re-export the derive macro when the derive feature is enabled  
#[cfg(feature = "derive")]
pub use bevy_repl_derive::ReplCommand;

pub mod prelude {
    pub use crate::{
        built_ins::{QuitCommand, CloseReplCommand},
        registry::ReplCommandRegistration,
    };
    pub use crate::{ReplPlugin, Repl, ReplCommand, ReplResult, ReplSet, ReplConfig};
    
    // Re-export derive macro in prelude
    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}

use anyhow::{Context, Result, anyhow, bail, ensure};
    
use std::{
    collections::BTreeMap,
};
use crate::terminal::BevyCrosstermTerminal;
use bevy::prelude::*;
use bevy_crossterm::prelude::*;


/// The main REPL plugin
#[derive(Default)]
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Add bevy_crossterm plugin first
        app.add_plugins(CrosstermPlugin);
        
        // Insert the configuration resource
        let config = self.config.clone();
        let mut repl = Repl::with_config(config.clone());
        if config.enabled_on_startup {
            repl.enable();
        }
        app.insert_resource(config)
            .insert_resource(repl)
            .insert_resource(ReplCommandRegistry::default())
            .configure_sets(Update, ReplSet::First); // Always runs

        // Add cleanup system
        app.add_systems(Update, exit_on_interrupt);
    }
}

/// This handles the crossterm interrupt event and exits the app.
/// 
/// Without this, the terminal might hang with no way to exit the app because
/// the exit event is not handled.
fn exit_on_interrupt(
    mut interrupt_events: EventReader<CrosstermInputEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for event in interrupt_events.read() {
        if let CrosstermInputEvent::Interrupt = event {
            exit.send(AppExit::Success);
        }
    }
}

// Use anyhow::Error directly - this is the standard pattern
pub type ReplResult<T> = Result<T, anyhow::Error>;

/// The SystemSet for console/command related systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReplSet {
    /// Systems that run before anything else, like toggling the REPL
    First,

    /// Systems operating the console UI (the input layer)
    Input,

    /// Systems executing console commands (the functionality layer).
    /// All command handler systems are added to this set
    Execution,

    /// Systems running after command systems, which depend on the fact commands have executed beforehand (the output layer).
    /// For example, a system which makes use of [`PrintReplLine`] events should be placed in this set
    Output,
}

pub struct ReplTerminal {
    terminal: Option<BevyCrosstermTerminal>,
    config: ReplConfig,
}

impl Default for ReplTerminal {
    fn default() -> Self {
        Self::new(ReplConfig::default())
    }
}

impl ReplTerminal {
    fn new(config: ReplConfig) -> Self {
        Self {
            terminal: None,
            config,
        }
    }
    
    fn spawn(&mut self) {
        if self.terminal.is_some() {
            return;
        }
        
        let mut terminal = BevyCrosstermTerminal::new(
            self.config.prompt.clone(),
            self.config.history_file.clone(),
        );
        
        if let Err(e) = terminal.init() {
            eprintln!("Failed to initialize terminal: {}", e);
            return;
        }
        
        self.terminal = Some(terminal);
    }
    
    fn shutdown(&mut self) {
        if let Some(mut terminal) = self.terminal.take() {
            terminal.cleanup().ok();
        }
    }
    
    fn try_recv_input(&mut self) -> Option<String> {
        if let Some(terminal) = &mut self.terminal {
            terminal.poll_input()
        } else {
            None
        }
    }
    
    fn send_output(&mut self, output: String) {
        if let Some(terminal) = &mut self.terminal {
            terminal.print_output(&output);
        }
    }
}

#[derive(Resource)]
pub struct Repl {
    enabled: bool,
    terminal: ReplTerminal,
}

impl Repl {
    /// Create a new REPL with default configuration
    pub fn new() -> Self {
        Self::with_config(ReplConfig::default())
    }

    /// Create a new REPL with custom configuration
    /// This allows users to customize the prompt, history file, and other settings
    pub fn with_config(config: ReplConfig) -> Self {
        Self {
            enabled: config.enabled_on_startup,
            terminal: ReplTerminal::new(config.clone()),
        }
    }
    
    /// Try to receive input from the terminal
    /// Returns None if no input is available (non-blocking)
    pub fn try_recv_input(&mut self) -> Option<String> {
        self.terminal.try_recv_input()
    }
    
    /// Send output to be printed by the terminal
    /// This prevents blocking the main Bevy thread during I/O
    pub fn send_output(&mut self, output: String) {
        self.terminal.send_output(output);
    }

    /// Check if the REPL is currently enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the REPL
    pub fn enable(&mut self) {
        self.enabled = true;
        // Initialize the terminal if it's not already running
        if self.terminal.terminal.is_none() {
            self.terminal.spawn();
        }
    }

    /// Disable the REPL
    pub fn disable(&mut self) {
        self.enabled = false;
        // Clean up the terminal
        self.terminal.shutdown();
    }

    /// Toggle the REPL between enabled and disabled states
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if self.enabled {
            self.enable();
        } else {
            self.disable();
        }
    }
}

/// Implement Drop to ensure graceful shutdown
/// This ensures the terminal is cleaned up properly and saves history
impl Drop for Repl {
    fn drop(&mut self) {
        // Clean up the terminal
        // This ensures history is saved before the program exits
        self.terminal.shutdown();
    }
}

/// Configuration for the REPL plugin
#[derive(Resource, Clone)]
pub struct ReplConfig {
    /// The prompt string to display
    pub prompt: String,

    /// The key to toggle the REPL. If None, the REPL is always enabled.
    pub toggle_key: Option<KeyCode>,

    /// Registered console commands
    pub commands: BTreeMap<&'static str, clap::Command>,

    /// Whether the REPL should be enabled when the app starts
    pub enabled_on_startup: bool,

    /// Custom history file path. If None, no history file is used
    /// This allows users to have separate history files for different Bevy apps
    pub history_file: Option<String>,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            toggle_key: None,
            commands: BTreeMap::new(),
            enabled_on_startup: true,
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

    /// Set a key to toggle the REPL on/off
    /// When set, pressing this key will toggle the REPL between enabled and disabled states
    /// Example: .with_toggle_key(KeyCode::F1)
    pub fn with_toggle_key(mut self, key: KeyCode) -> Self {
        self.toggle_key = Some(key);
        self
    }

    /// Set a custom history file path
    /// This allows different Bevy apps to have separate command histories
    /// Example: .with_history_file(".my_game_history")
    pub fn with_history_file(mut self, history_file: impl Into<String>) -> Self {
        self.history_file = Some(history_file.into());
        self
    }
}
