use bevy::prelude::*;
use bevy_crossterm::prelude::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

/// Terminal state management for the REPL using bevy_crossterm
#[derive(Resource)]
pub struct BevyCrosstermTerminal {
    input_queue: Arc<Mutex<VecDeque<String>>>,
    output_queue: Arc<Mutex<VecDeque<String>>>,
    prompt: String,
    history_file: Option<String>,
    history: VecDeque<String>,
    current_line: String,
    cursor_position: usize,
    is_active: bool,
}

impl BevyCrosstermTerminal {
    pub fn new(prompt: String, history_file: Option<String>) -> Self {
        Self {
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(VecDeque::new())),
            prompt,
            history_file,
            history: VecDeque::new(),
            current_line: String::new(),
            cursor_position: 0,
            is_active: false,
        }
    }

    /// Initialize the terminal
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Load history if specified
        if let Some(ref history_path) = self.history_file {
            if let Ok(contents) = std::fs::read_to_string(history_path) {
                for line in contents.lines() {
                    if !line.trim().is_empty() {
                        self.history.push_back(line.to_string());
                    }
                }
            }
        }
        
        self.is_active = true;
        Ok(())
    }

    /// Clean up terminal state
    pub fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Save history if specified
        if let Some(ref history_path) = self.history_file {
            let history_content: String = self.history.iter()
                .take(1000) // Limit history size
                .map(|line| format!("{}\n", line))
                .collect();
            std::fs::write(history_path, history_content).ok();
        }
        
        self.is_active = false;
        Ok(())
    }

    /// Handle input events from bevy_crossterm
    pub fn handle_input(&mut self, input: &str) {
        if !self.is_active {
            return;
        }

        // Process input character by character
        for c in input.chars() {
            match c {
                '\n' | '\r' => {
                    // Enter key - submit current line
                    let line = self.current_line.clone();
                    if !line.trim().is_empty() {
                        self.add_to_history(line.clone());
                        let mut queue = self.input_queue.lock().unwrap();
                        queue.push_back(line);
                    }
                    self.current_line.clear();
                    self.cursor_position = 0;
                }
                '\x03' => {
                    // Ctrl+C - send quit command
                    let mut queue = self.input_queue.lock().unwrap();
                    queue.push_back("quit".to_string());
                }
                '\x04' => {
                    // Ctrl+D - send quit command
                    let mut queue = self.input_queue.lock().unwrap();
                    queue.push_back("quit".to_string());
                }
                '\x08' | '\x7f' => {
                    // Backspace
                    if self.cursor_position > 0 {
                        self.current_line.remove(self.cursor_position - 1);
                        self.cursor_position -= 1;
                    }
                }
                '\x1b' => {
                    // Escape sequence - ignore for now
                    continue;
                }
                _ if c.is_ascii_control() => {
                    // Other control characters - ignore
                    continue;
                }
                _ => {
                    // Regular character
                    if self.cursor_position < self.current_line.len() {
                        self.current_line.insert(self.cursor_position, c);
                    } else {
                        self.current_line.push(c);
                    }
                    self.cursor_position += 1;
                }
            }
        }
    }

    /// Poll for input (non-blocking)
    pub fn poll_input(&mut self) -> Option<String> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.pop_front()
    }

    /// Get the current line buffer
    pub fn get_current_line(&self) -> String {
        self.current_line.clone()
    }

    /// Clear the current line
    pub fn clear_line(&mut self) {
        self.current_line.clear();
        self.cursor_position = 0;
    }

    /// Add a command to history
    pub fn add_to_history(&mut self, command: String) {
        if !command.trim().is_empty() && 
           self.history.back().map(|last| last != &command).unwrap_or(true) {
            self.history.push_back(command);
        }
    }

    /// Print output
    pub fn print_output(&mut self, output: &str) {
        let mut queue = self.output_queue.lock().unwrap();
        queue.push_back(output.to_string());
    }

    /// Process output queue
    pub fn process_output(&mut self) {
        let mut queue = self.output_queue.lock().unwrap();
        while let Some(output) = queue.pop_front() {
            // Output will be handled by bevy_crossterm rendering system
            println!("{}", output);
        }
    }

    /// Get the prompt text
    pub fn get_prompt(&self) -> String {
        format!("{}{}", self.prompt, self.current_line)
    }

    /// Get cursor position relative to prompt
    pub fn get_cursor_position(&self) -> usize {
        self.prompt.len() + self.cursor_position
    }

    /// Handle Ctrl+C interruption
    pub fn handle_interrupt(&mut self) {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back("quit".to_string());
    }

    /// Force quit the application
    pub fn force_quit(&mut self) {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back("quit".to_string());
    }
}

impl Drop for BevyCrosstermTerminal {
    fn drop(&mut self) {
        self.cleanup().ok();
    }
} 
