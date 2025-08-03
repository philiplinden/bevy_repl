#![doc = include_str ! ("../README.md")]

pub mod built_ins;
pub mod error;
pub mod registry;
pub mod input;
pub mod execution;
pub mod output;
pub mod terminal;

// Re-export the derive macro when the derive feature is enabled  
#[cfg(feature = "derive")]
pub use bevy_repl_derive::ReplCommand;

pub mod prelude {
    pub use crate::{
        built_ins::{QuitCommand, CloseReplCommand},
        registry::ReplCommandRegistration,
    };
    pub use crate::{ReplPlugin, Repl, ReplCommand, ReplResult, ReplSet, repl_enabled, ReplConfig, ReplEnableEvent, ReplDisableEvent, ReplToggleEvent};
    
    // Re-export derive macro in prelude
    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}

use crate::{
    input::ReplInputPlugin,
    registry::{ReplCommandRegistry, ReplCommandRegistration},
    output::ReplOutputPlugin,
    execution::ReplExecutionPlugin,
};
    
use std::{
    collections::BTreeMap,
};
use crate::terminal::BevyCrosstermTerminal;
use bevy::prelude::*;
use bevy_crossterm::prelude::*;


/// The main REPL plugin
pub struct ReplPlugin {
    config: ReplConfig,
    no_built_in_commands: bool,
}

impl Default for ReplPlugin {
    fn default() -> Self {
        Self {
            config: ReplConfig::default(),
            no_built_in_commands: false,
        }
    }
}

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
            .configure_sets(Update, ReplSet::First) // Always runs
            .add_systems(Update, toggle_repl_system.in_set(ReplSet::First))
            .add_observer(repl_enable_observer)
            .add_observer(repl_disable_observer)
            .add_observer(repl_toggle_observer)
            .add_plugins((
                ReplInputPlugin,
                ReplOutputPlugin,
                ReplExecutionPlugin,
            ));

        if !self.no_built_in_commands {
            app.add_repl_command::<built_ins::CloseReplCommand>();
            app.add_repl_command::<built_ins::QuitCommand>();
        }
        
        // Add cleanup system
        app.add_systems(Update, cleanup_on_exit);
    }
}

/// Run condition which does not run any REPL systems if it is disabled
pub fn repl_enabled(repl: Res<Repl>) -> bool {
    repl.is_enabled()
}

/// Event to enable the REPL
#[derive(Event)]
pub struct ReplEnableEvent;

/// Event to disable the REPL
#[derive(Event)]
pub struct ReplDisableEvent;

/// Event to toggle the REPL between enabled and disabled states
#[derive(Event)]
pub struct ReplToggleEvent;


/// System that handles keyboard input for toggling the REPL
/// This listens for the configured toggle key and triggers a toggle event
pub(crate) fn toggle_repl_system(
    keyboard_input: Option<Res<ButtonInput<KeyCode>>>,
    config: Res<ReplConfig>,
    mut commands: Commands,
) {
    // Only process toggle if input resource exists and a key is configured
    if let (Some(keyboard_input), Some(toggle_key)) = (keyboard_input, config.toggle_key) {
        if keyboard_input.just_pressed(toggle_key) {
            commands.trigger(ReplToggleEvent);
        }
    }
}

/// Observer that handles REPL enable events
fn repl_enable_observer(
    _trigger: Trigger<ReplEnableEvent>,
    mut repl: ResMut<Repl>,
) {
    info!("Starting Bevy REPL...");
    info!("Type 'help' for available commands, 'quit' to exit.");
    repl.enable();
}

/// Observer that handles REPL disable events
fn repl_disable_observer(
    _trigger: Trigger<ReplDisableEvent>,
    mut repl: ResMut<Repl>,
) {
    info!("Stopping Bevy REPL...");
    repl.disable();
}

/// Observer that handles REPL toggle events
fn repl_toggle_observer(
    _trigger: Trigger<ReplToggleEvent>,
    mut repl: ResMut<Repl>,
) {
    repl.toggle();

    // Log the state change
    if repl.is_enabled() {
        info!("REPL enabled");
    } else {
        info!("REPL disabled");
    }
}

/// Trigger the REPL to be enabled
/// Send this event to enable the REPL through the trigger system
pub fn enable_repl(mut commands: Commands) {
    commands.trigger(ReplEnableEvent);
}

/// Trigger the REPL to be disabled  
/// Send this event to disable the REPL through the trigger system
pub fn disable_repl(mut commands: Commands) {
    commands.trigger(ReplDisableEvent);
}

/// Trigger the REPL to toggle between enabled and disabled states
/// Send this event to toggle the REPL through the trigger system
pub fn toggle_repl(mut commands: Commands) {
    commands.trigger(ReplToggleEvent);
}

/// System that handles cleanup when the app is about to exit
fn cleanup_on_exit(
    mut exit_events: EventReader<AppExit>,
    mut repl: ResMut<Repl>,
) {
    for _event in exit_events.read() {
        info!("Cleaning up REPL before exit...");
        repl.disable();
    }
}

/// Trait for REPL commands
pub trait ReplCommand: Send + Sync + 'static {
    fn command(&self) -> clap::Command;
    
    /// Execute command
    fn execute(&self, _commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        Ok("Command not implemented".to_string())
    }
}

pub type ReplResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
/// The SystemSet for console/command related systems
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
