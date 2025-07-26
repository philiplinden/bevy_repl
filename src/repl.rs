use std::sync::{mpsc::{self, Receiver, Sender}, Mutex};
use std::thread;

use bevy::prelude::*;
use crate::commands::CommandRegistry;

// Resource to hold the command queue safely
#[derive(Resource)]
pub struct CommandQueue(pub Mutex<Receiver<String>>);

// System to execute commands from the queue
pub fn command_executor(
    queue: Res<CommandQueue>,
    registry: Res<CommandRegistry>,
    mut exit_events: EventWriter<AppExit>,
) {
    let queue = queue.0.lock().unwrap();
    while let Ok(command) = queue.try_recv() {
        match registry.execute(&command, &mut exit_events) {
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

// TUI plugin skeleton
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, command_executor);
    }
}

// Function to spawn the REPL input thread
pub fn start_repl_input(tx: Sender<String>) {
    thread::spawn(move || {
        use std::io::{self, Write};
        loop {
            print!("> ");
            // Terminal I/O in separate thread
        }
    });
}

// Add to your ReplConfiguration
#[derive(Resource, Clone)]
pub struct ReplConfiguration {
    pub prompt: String,
    pub enabled: bool,
    // Terminal styling options
    pub colors: TerminalColors,
    pub show_timestamps: bool,
    pub max_output_width: usize,
}

#[derive(Clone)]
pub struct TerminalColors {
    pub prompt: &'static str,    // ANSI color codes
    pub success: &'static str,
    pub error: &'static str,
    pub info: &'static str,
    pub warning: &'static str,
}
