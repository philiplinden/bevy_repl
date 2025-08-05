//! This module creates a TUI with `bevy_ratatui` that mimics a terminal.
//! 
//! A single line at the bottom of the TUI is the input area, where text can be
//! edited in place. This is the prompt area of the REPL. Above the input area
//! is a scrollable area where stdout and stderr are captured and displayed.
//!
//! ```text
//! ┌───your terminal─────────────────────────────────────────────────────────┐
//! │ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL                │
//! │ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Type 'help' for commands     │
//! │                                                                         │
//! │ [Game logs and command output appear here...]                           │
//! │                                                                         │
//! │ > spawn-player Bob                                                      │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

use bevy::prelude::*;
use std::collections::VecDeque;

/// Terminal state management for the REPL using bevy_ratatui
#[derive(Resource)]
pub struct BevyRatatuiTerminal {
    /// Scrollable log output
    log_lines: VecDeque<String>,
    /// Current input line
    current_line: String,
    /// Cursor position in input line
    cursor_position: usize,
    /// Terminal prompt
    prompt: String,
    /// Maximum number of log lines to keep
    max_log_lines: usize,
    /// Whether the terminal is active
    is_active: bool,
}

impl Default for BevyRatatuiTerminal {
    fn default() -> Self {
        Self {
            log_lines: VecDeque::new(),
            current_line: String::new(),
            cursor_position: 0,
            prompt: "> ".to_string(),
            max_log_lines: 1000,
            is_active: true,
        }
    }
}

impl BevyRatatuiTerminal {
    /// Create a new terminal with custom prompt
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            ..Default::default()
        }
    }

    /// Add a line to the log output
    pub fn add_log_line(&mut self, line: String) {
        self.log_lines.push_back(line);
        
        // Trim old lines if we exceed the maximum
        while self.log_lines.len() > self.max_log_lines {
            self.log_lines.pop_front();
        }
    }

    /// Get the current input line
    pub fn get_current_line(&self) -> &str {
        &self.current_line
    }

    /// Set the current input line
    pub fn set_current_line(&mut self, line: String) {
        self.current_line = line;
        self.cursor_position = self.current_line.len();
    }

    /// Clear the current input line
    pub fn clear_line(&mut self) {
        self.current_line.clear();
        self.cursor_position = 0;
    }

    /// Handle character input
    pub fn handle_char(&mut self, c: char) {
        if !self.is_active {
            return;
        }

        match c {
            '\n' | '\r' => {
                // Enter key - submit current line
                let line = self.current_line.clone();
                if !line.trim().is_empty() {
                    self.add_log_line(format!("{}{}", self.prompt, line));
                    // TODO: Send command for processing
                }
                self.clear_line();
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
                return;
            }
            _ if c.is_ascii_control() => {
                // Other control characters - ignore
                return;
            }
            _ => {
                // Regular character
                self.current_line.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
        }
    }

    // /// Handle special keys
    // pub fn handle_key(&mut self, key: ratatui::event::KeyEvent) {
    //     if !self.is_active {
    //         return;
    //     }

    //     match key.code {
    //         ratatui::event::KeyCode::Left => {
    //             if self.cursor_position > 0 {
    //                 self.cursor_position -= 1;
    //             }
    //         }
    //         ratatui::event::KeyCode::Right => {
    //             if self.cursor_position < self.current_line.len() {
    //                 self.cursor_position += 1;
    //             }
    //         }
    //         ratatui::event::KeyCode::Home => {
    //             self.cursor_position = 0;
    //         }
    //         ratatui::event::KeyCode::End => {
    //             self.cursor_position = self.current_line.len();
    //         }
    //         ratatui::event::KeyCode::Up => {
    //             // TODO: Navigate command history
    //         }
    //         ratatui::event::KeyCode::Down => {
    //             // TODO: Navigate command history
    //         }
    //         _ => {}
    //     }
    // }

    /// Get the cursor position in the input line
    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Set the terminal prompt
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    /// Get the terminal prompt
    pub fn get_prompt(&self) -> &str {
        &self.prompt
    }

    /// Toggle terminal active state
    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
    }

    /// Check if terminal is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Plugin to set up the terminal TUI
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BevyRatatuiTerminal>();
    }
}

