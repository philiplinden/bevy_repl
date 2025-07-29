use bevy::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;
use crate::config::ReplConfig;

pub trait ReplCommand: Send + Sync + 'static {
    fn command(&self) -> clap::Command;
    fn execute(&self, world: &mut World, matches: &clap::ArgMatches) -> ReplResult<String>;
    fn name(&self) -> &'static str;
}

pub type ReplResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
/// The SystemSet for console/command related systems
pub enum ReplSet {
    /// Systems configuring commands at startup, only once
    Startup,

    /// Systems operating the console UI (the input layer)
    ReplUI,

    /// Systems executing console commands (the functionality layer).
    /// All command handler systems are added to this set
    Commands,

    /// Systems running after command systems, which depend on the fact commands have executed beforehand (the output layer).
    /// For example, a system which makes use of [`PrintReplLine`] events should be placed in this set
    PostCommands,
}


#[derive(Resource, Default)]
pub(crate) struct ReplState {
    #[cfg(feature = "history")]
    pub(crate) history: ReplHistory,
    #[cfg(feature = "suggestions")]
    pub(crate) suggestions: ReplSuggestions,
}


#[derive(Resource)]
pub struct Repl {
    input_receiver: Arc<Mutex<Receiver<String>>>,
    output_sender: Sender<String>,
    should_quit: Arc<AtomicBool>,
    _input_handle: JoinHandle<()>,
}

impl Repl {
    /// Create a new REPL with default configuration
    pub fn new() -> Self {
        Self::with_config(ReplConfig::default())
    }

    /// Create a new REPL with custom configuration
    /// This allows users to customize the prompt, history file, and other settings
    pub fn with_config(config: ReplConfig) -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let should_quit = Arc::new(AtomicBool::new(false));
        
        // Clone config values for the input thread
        let prompt = config.prompt.clone();
        let history_file = config.history_file.clone();
        
        let quit_flag = should_quit.clone();
        let input_handle = thread::spawn(move || {
            // Create rustyline editor with custom configuration if needed
            let mut rl = rustyline::DefaultEditor::new().unwrap();
            
            // Load custom history file if specified
            // This allows different Bevy apps to have separate command histories
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
                            // Add command to history - rustyline will save this automatically
                            rl.add_history_entry(&line).ok();
                            if input_tx.send(line).is_err() {
                                break; // Main thread died
                            }
                        }
                    }
                    Err(_) => break, // EOF or error
                }
            }
            
            // Save history on thread exit
            // This ensures history is persisted even if the app crashes
            if let Some(ref history_path) = history_file {
                if let Err(e) = rl.save_history(history_path) {
                    eprintln!("Warning: Could not save history to {}: {}", history_path, e);
                }
            }
            // If no custom history file, rustyline saves to default location automatically
        });
        
        // Spawn output handler thread
        // This thread prints REPL output without blocking the main Bevy thread
        thread::spawn(move || {
            while let Ok(output) = output_rx.recv() {
                println!("{}", output);
            }
        });
        
        Self {
            input_receiver: Arc::new(Mutex::new(input_rx)),
            output_sender: output_tx,
            should_quit,
            _input_handle: input_handle,
        }
    }
    
    /// Try to receive input from the rustyline thread
    /// Returns None if no input is available (non-blocking)
    pub fn try_recv_input(&self) -> Option<String> {
        self.input_receiver.lock().unwrap().try_recv().ok()
    }
    
    /// Send output to be printed by the output thread
    /// This prevents blocking the main Bevy thread during I/O
    pub fn send_output(&self, output: String) {
        let _ = self.output_sender.send(output);
    }
    
    /// Request graceful shutdown of the REPL
    /// This sets the quit flag, which the input thread will check and exit cleanly
    pub fn request_quit(&self) {
        self.should_quit.store(true, Ordering::Relaxed);
    }
    
    /// Check if the REPL has been requested to quit
    pub fn should_quit(&self) -> bool {
        self.should_quit.load(Ordering::Relaxed)
    }
}

/// Implement Drop to ensure graceful shutdown
/// This ensures the rustyline thread exits cleanly and saves history
impl Drop for Repl {
    fn drop(&mut self) {
        // Request the input thread to quit
        self.request_quit();
        
        // Wait for the input thread to finish
        // This ensures history is saved before the program exits
        if let Err(e) = self._input_handle.join() {
            eprintln!("Warning: Input thread panicked: {:?}", e);
        }
    }
}
