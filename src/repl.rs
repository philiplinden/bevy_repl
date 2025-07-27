use bevy::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;

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
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let should_quit = Arc::new(AtomicBool::new(false));
        
        let quit_flag = should_quit.clone();
        let input_handle = thread::spawn(move || {
            let mut rl = rustyline::DefaultEditor::new().unwrap();
            
            while !quit_flag.load(Ordering::Relaxed) {
                match rl.readline("bevy> ") {
                    Ok(line) => {
                        if !line.trim().is_empty() {
                            rl.add_history_entry(&line).ok();
                            if input_tx.send(line).is_err() {
                                break; // Main thread died
                            }
                        }
                    }
                    Err(_) => break, // EOF or error
                }
            }
        });
        
        // Spawn output handler
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
    
    pub fn try_recv_input(&self) -> Option<String> {
        self.input_receiver.lock().unwrap().try_recv().ok()
    }
    
    pub fn send_output(&self, output: String) {
        let _ = self.output_sender.send(output);
    }
    
    pub fn request_quit(&self) {
        self.should_quit.store(true, Ordering::Relaxed);
    }
    
    pub fn should_quit(&self) -> bool {
        self.should_quit.load(Ordering::Relaxed)
    }
}
