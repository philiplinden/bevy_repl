#![doc = include_str ! ("../README.md")]

pub mod built_ins;
pub mod error;
pub mod registry;
pub mod input;
pub mod execution;
pub mod output;

// Re-export the derive macro when the derive feature is enabled  
#[cfg(feature = "derive")]
pub use bevy_repl_derive::ReplCommand;

pub mod prelude {
    #[cfg(feature = "diagnostics")]
    pub use crate::built_ins::SysInfoCommand;
    pub use crate::{
        built_ins::{HelpCommand, QuitCommand, CloseReplCommand},
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
    sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex, atomic::{AtomicBool, Ordering}},
    thread::{self, JoinHandle},
    collections::BTreeMap,
};
use bevy::prelude::*;


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
            app.add_repl_command::<built_ins::HelpCommand>();
            app.add_repl_command::<built_ins::CloseReplCommand>();
            app.add_repl_command::<built_ins::QuitCommand>();
            #[cfg(feature = "diagnostics")]
            app.add_repl_command::<built_ins::SysInfoCommand>();
        }
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<ReplConfig>,
    mut commands: Commands,
) {
    // Only process toggle if a key is configured
    if let Some(toggle_key) = config.toggle_key {
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

/// Trait for REPL commands
/// Use `execute` for commands that don't need world access
/// Use `execute_with_world` for commands that need world access
pub trait ReplCommand: Send + Sync + 'static {
    fn command(&self) -> clap::Command;
    
    /// Execute command with limited access (for simple commands)
    fn execute(&self, _commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        // Default implementation for commands that don't need world access
        Ok("Command not implemented".to_string())
    }
    
    /// Execute command with full world access (for complex commands)
    fn execute_with_world(&self, _world: &World, commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        // Default: fall back to regular execute
        self.execute(commands, matches)
    }
    
    /// Whether this command needs world access
    fn needs_world_access(&self) -> bool {
        false
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

pub struct ReplThreadManager {
    input_handle: Option<JoinHandle<()>>,
    should_quit: Arc<AtomicBool>,
    input_receiver: Arc<Mutex<Receiver<String>>>,
    output_sender: Sender<String>,
    config: ReplConfig,
}

impl Default for ReplThreadManager {
    fn default() -> Self {
        Self::new(ReplConfig::default())
    }
}

impl ReplThreadManager {
    fn new(config: ReplConfig) -> Self {
        let (_input_tx, input_rx) = mpsc::channel();
        let (output_tx, _output_rx) = mpsc::channel();
        
        Self {
            input_handle: None,
            should_quit: Arc::new(AtomicBool::new(false)),
            input_receiver: Arc::new(Mutex::new(input_rx)),
            output_sender: output_tx,
            config: config,
        }
    }
    
    fn spawn(&mut self) {
        if self.input_handle.is_some() {
            return;
        }
        
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let quit_flag = self.should_quit.clone();
        
        // Update the receiver to use the new channel
        self.input_receiver = Arc::new(Mutex::new(input_rx));
        self.output_sender = output_tx;
        let history_file = self.config.history_file.clone();
        let prompt = self.config.prompt.clone();
        
        let handle = thread::spawn(move || {
            let mut rl = rustyline::DefaultEditor::new().unwrap();
            
            // Load custom history file if specified
            if let Some(ref history_path) = history_file {
                if let Err(e) = rl.load_history(history_path) {
                    // Don't fail if history file doesn't exist yet
                    if !e.to_string().contains("No such file") {
                        eprintln!("Warning: Could not load history from {}: {}", history_path, e);
                    }
                }
            }
            
            while !quit_flag.load(Ordering::Relaxed) {
                match rl.readline(&prompt) {
                    Ok(line) => {
                        if !line.trim().is_empty() {
                            rl.add_history_entry(&line).ok();
                            if input_tx.send(line).is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            
            // Save history on thread exit
            if let Some(ref history_path) = history_file {
                if let Err(e) = rl.save_history(history_path) {
                    eprintln!("Warning: Could not save history to {}: {}", history_path, e);
                }
            }
        });
        
        // Spawn output thread
        thread::spawn(move || {
            while let Ok(output) = output_rx.recv() {
                println!("{}", output);
            }
        });
        
        self.input_handle = Some(handle);
    }
    
    fn shutdown(&mut self) {
        self.should_quit.store(true, Ordering::Relaxed);
        if let Some(handle) = self.input_handle.take() {
            let _ = handle.join();
        }
    }
}

#[derive(Resource)]
pub struct Repl {
    enabled: bool,
    thread_manager: ReplThreadManager,
    config: ReplConfig,
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
            thread_manager: ReplThreadManager::new(config.clone()),
            config: config,
        }
    }
    
    /// Try to receive input from the rustyline thread
    /// Returns None if no input is available (non-blocking)
    pub fn try_recv_input(&self) -> Option<String> {
        self.thread_manager.input_receiver.lock().unwrap().try_recv().ok()
    }
    
    /// Send output to be printed by the output thread
    /// This prevents blocking the main Bevy thread during I/O
    pub fn send_output(&self, output: String) {
        let _ = self.thread_manager.output_sender.send(output);
    }

    /// Check if the REPL is currently enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the REPL
    pub fn enable(&mut self) {
        self.enabled = true;
        // Spawn the rustyline thread if it's not already running
        if self.thread_manager.input_handle.is_none() {
            self.thread_manager.spawn();
        }
    }

    /// Disable the REPL
    pub fn disable(&mut self) {
        self.enabled = false;
        // Set the quit flag to signal the rustyline thread to exit
        self.thread_manager.shutdown();
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
/// This ensures the rustyline thread exits cleanly and saves history
impl Drop for Repl {
    fn drop(&mut self) {
        // Wait for the input thread to finish
        // This ensures history is saved before the program exits
        self.thread_manager.shutdown();
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

    /// Custom history file path. If None, uses rustyline's default (~/.rustyline_history)
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

    #[cfg(feature = "custom-history-file")]
    /// Set a custom history file path
    /// This allows different Bevy apps to have separate command histories
    /// Example: .with_history_file(".my_game_history")
    pub fn with_history_file(mut self, history_file: impl Into<String>) -> Self {
        self.history_file = Some(history_file.into());
        self
    }
}
