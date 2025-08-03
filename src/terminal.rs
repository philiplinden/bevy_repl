use crossterm::{
    cursor::{self, MoveToColumn},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use bevy::prelude::*;
use std::{
    collections::VecDeque,
    io::stdout,
    time::Duration,
};

/// Terminal state management for the REPL
#[derive(Resource)]
pub struct CrosstermTerminal {
    prompt_buffer: String,
    cursor_position: usize,
    history: VecDeque<String>,
    history_index: usize,
    prompt: String,
    history_file: Option<String>,
    is_raw_mode: bool,
}

impl CrosstermTerminal {
    pub fn new(prompt: String, history_file: Option<String>) -> Self {
        Self {
            prompt_buffer: String::new(),
            cursor_position: 0,
            history: VecDeque::new(),
            history_index: 0,
            prompt,
            history_file,
            is_raw_mode: false,
        }
    }

    /// Initialize the terminal for raw mode
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        terminal::enable_raw_mode()?;
        self.is_raw_mode = true;
        
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
        
        self.print_prompt();
        Ok(())
    }

    /// Clean up terminal state
    pub fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.is_raw_mode {
            terminal::disable_raw_mode()?;
            self.is_raw_mode = false;
        }
        
        // Save history if specified
        if let Some(ref history_path) = self.history_file {
            let history_content: String = self.history.iter()
                .take(1000) // Limit history size
                .map(|line| format!("{}\n", line))
                .collect();
            std::fs::write(history_path, history_content).ok();
        }
        
        Ok(())
    }

    /// Poll for input events (non-blocking)
    pub fn poll_event(&mut self) -> Result<Option<Event>, Box<dyn std::error::Error + Send + Sync>> {
        if event::poll(Duration::from_millis(0))? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    /// Get the current line buffer
    pub fn get_current_line(&self) -> String {
        self.prompt_buffer.clone()
    }

    /// Clear the current line and reset cursor
    pub fn clear_line(&mut self) {
        self.prompt_buffer.clear();
        self.cursor_position = 0;
        self.history_index = 0;
    }

    /// Add a command to history
    pub fn add_to_history(&mut self, command: String) {
        // Don't add empty commands or duplicates
        if !command.trim().is_empty() && 
           self.history.back().map(|last| last != &command).unwrap_or(true) {
            self.history.push_back(command);
        }
    }

    /// Print the prompt
    pub fn print_prompt(&mut self) {
        execute!(
            stdout(),
            Print(&self.prompt),
            Print(&self.prompt_buffer),
        ).ok();
    }

    /// Handle a key event
    pub fn handle_key(&mut self, key_code: KeyCode, modifiers: KeyModifiers) -> Option<String> {
        match key_code {
            KeyCode::Char(c) => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    self.handle_ctrl_char(c)
                } else {
                    self.insert_char(c);
                    None
                }
            }
            KeyCode::Backspace => {
                self.backspace();
                None
            }
            KeyCode::Delete => {
                self.delete();
                None
            }
            KeyCode::Left => {
                self.move_cursor_left();
                None
            }
            KeyCode::Right => {
                self.move_cursor_right();
                None
            }
            KeyCode::Up => {
                self.history_up();
                None
            }
            KeyCode::Down => {
                self.history_down();
                None
            }
            KeyCode::Enter => {
                let line = self.get_current_line();
                self.clear_line();
                self.print_prompt();
                Some(line)
            }
            KeyCode::Tab => {
                // TODO: Implement tab completion
                None
            }
            _ => None,
        }
    }

    /// Handle Ctrl+key combinations
    fn handle_ctrl_char(&mut self, c: char) -> Option<String> {
        match c {
            'c' => {
                // Ctrl+C - clear line
                self.clear_line();
                self.print_prompt();
                None
            }
            'l' => {
                // Ctrl+L - clear screen
                execute!(stdout(), Clear(ClearType::All)).ok();
                self.print_prompt();
                None
            }
            'u' => {
                // Ctrl+U - clear line before cursor
                self.clear_before_cursor();
                None
            }
            'k' => {
                // Ctrl+K - clear line after cursor
                self.clear_after_cursor();
                None
            }
            _ => None,
        }
    }

    /// Insert a character at cursor position
    fn insert_char(&mut self, c: char) {
        if self.cursor_position < self.prompt_buffer.len() {
            self.prompt_buffer.insert(self.cursor_position, c);
        } else {
            self.prompt_buffer.push(c);
        }
        self.cursor_position += 1;
        self.redraw_line();
    }

    /// Delete character before cursor
    fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.prompt_buffer.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.redraw_line();
        }
    }

    /// Delete character at cursor
    fn delete(&mut self) {
        if self.cursor_position < self.prompt_buffer.len() {
            self.prompt_buffer.remove(self.cursor_position);
            self.redraw_line();
        }
    }

    /// Move cursor left
    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            execute!(stdout(), cursor::MoveLeft(1)).ok();
        }
    }

    /// Move cursor right
    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.prompt_buffer.len() {
            self.cursor_position += 1;
            execute!(stdout(), cursor::MoveRight(1)).ok();
        }
    }

    /// Navigate up in history
    fn history_up(&mut self) {
        if self.history_index < self.history.len() {
            if self.history_index == 0 {
                // Save current line as temporary
                if !self.prompt_buffer.is_empty() {
                    self.history.push_front(self.prompt_buffer.clone());
                }
            }
            self.history_index += 1;
            let history_line = self.history.get(self.history.len() - self.history_index).unwrap();
            self.prompt_buffer = history_line.clone();
            self.cursor_position = self.prompt_buffer.len();
            self.redraw_line();
        }
    }

    /// Navigate down in history
    fn history_down(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if self.history_index == 0 {
                // Restore original line
                if let Some(original) = self.history.pop_front() {
                    self.prompt_buffer = original;
                }
            } else {
                let history_line = self.history.get(self.history.len() - self.history_index).unwrap();
                self.prompt_buffer = history_line.clone();
            }
            self.cursor_position = self.prompt_buffer.len();
            self.redraw_line();
        }
    }

    /// Clear line before cursor
    fn clear_before_cursor(&mut self) {
        if self.cursor_position > 0 {
            self.prompt_buffer.drain(0..self.cursor_position);
            self.cursor_position = 0;
            self.redraw_line();
        }
    }

    /// Clear line after cursor
    fn clear_after_cursor(&mut self) {
        if self.cursor_position < self.prompt_buffer.len() {
            self.prompt_buffer.truncate(self.cursor_position);
            self.redraw_line();
        }
    }

    /// Redraw the entire line
    fn redraw_line(&mut self) {
        // Move to beginning of line
        execute!(
            stdout(),
            MoveToColumn(0),
            Clear(ClearType::FromCursorDown),
            Print(&self.prompt),
            Print(&self.prompt_buffer),
        ).ok();
        
        // Move cursor to correct position
        let target_pos = self.prompt.len() + self.cursor_position;
        execute!(stdout(), MoveToColumn(target_pos as u16)).ok();
    }

    /// Print output without interfering with input
    pub fn print_output(&mut self, output: &str) {
        // Move to new line and print output
        execute!(
            stdout(),
            Print("\n"),
            Print(output),
            Print("\n"),
            Print(&self.prompt),
            Print(&self.prompt_buffer),
        ).ok();
        
        // Restore cursor position
        let target_pos = self.prompt.len() + self.cursor_position;
        execute!(stdout(), MoveToColumn(target_pos as u16)).ok();
    }
}

impl Drop for CrosstermTerminal {
    fn drop(&mut self) {
        self.cleanup().ok();
    }
} 
