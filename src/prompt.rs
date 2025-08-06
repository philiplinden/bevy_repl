use bevy::prelude::*;
use bevy_ratatui::{
    crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind},
    event::KeyEvent,
    crossterm::{
        cursor::MoveTo,
        terminal::{self, Clear, ClearType},
        ExecutableCommand,
    },
};
use std::io::{stdout, Write};

use crate::repl::{Repl, ReplEvent, repl_is_enabled};

#[derive(Resource, Clone)]
pub struct PromptPlugin {
    /// The prompt to display in the REPL console to the left of the input area.
    pub prompt: String,
    /// Enable a border around the REPL console.
    pub border: bool,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            border: true,
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplPrompt {
            symbol: Some(self.prompt.clone()),
            lines: Vec::new(),
        });
        app.add_event::<ParseReplBufferEvent>();
        app.add_systems(Update, buffer_input.run_if(repl_is_enabled));
        app.add_systems(Update, display_prompt.run_if(repl_buffer_changed.and(repl_is_enabled)));
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub lines: Vec<String>,
}


#[derive(Event)]
pub struct ParseReplBufferEvent {
    pub buffer: String,
}

fn repl_buffer_changed(repl: Res<Repl>) -> bool {
    repl.is_changed()
}

fn buffer_input(
    mut repl: ResMut<Repl>,
    mut crossterm_key_events: EventReader<KeyEvent>,
    mut parse_input_buffer_events: EventWriter<ParseReplBufferEvent>,
) {
    for event in crossterm_key_events.read() {
        if event.kind == CrosstermKeyEventKind::Press {
            match event.code {
                CrosstermKeyCode::Enter => {
                    parse_input_buffer_events.write(ParseReplBufferEvent {
                        buffer: repl.drain_buffer(),
                    });
                }
                CrosstermKeyCode::Char(c) => {
                    repl.push(c);
                }
                CrosstermKeyCode::Backspace => {
                    repl.backspace();
                }
                CrosstermKeyCode::Left => {
                    repl.left();
                }
                CrosstermKeyCode::Right => {
                    repl.right();
                }
                CrosstermKeyCode::Home => {
                    repl.home();
                }
                CrosstermKeyCode::End => {
                    repl.end();
                }
                CrosstermKeyCode::Delete => {
                    repl.delete();
                }
                _ => { /* ignore other non-character keys */ }
            }
        }
    }
}

/// System that displays the current input buffer at the bottom of the terminal
fn display_prompt(repl: Res<Repl>, prompt: Res<ReplPrompt>) {
    // Get terminal size
    let (width, height) = match terminal::size() {
        Ok(size) => size,
        Err(_) => return, // If we can't get terminal size, skip rendering
    };
    
    // Calculate the prompt line (bottom of terminal)
    let prompt_line = height.saturating_sub(1);
    
    // Display the prompt and current buffer
    let prompt_text = if let Some(symbol) = prompt.symbol.clone() {
        format!("{}{}", symbol, repl.buffer)
    } else {
        repl.buffer.clone()
    };
    
    // If the prompt text is longer than the terminal width, split it into multiple lines
    let mut display_lines = Vec::new();
    let mut start = 0;
    let prompt_len = prompt_text.len();
    let width_usize = width as usize;
    while start < prompt_len {
        let end = (start + width_usize).min(prompt_len);
        display_lines.push(&prompt_text[start..end]);
        start = end;
    }
    // Position cursor at the correct location within the buffer
    let cursor_x = (prompt_text.len() + repl.cursor_pos) as u16;
    let cursor_x = cursor_x.min(width.saturating_sub(1));
    
    // Execute terminal operations sequentially to avoid borrow checker issues
    let _ = stdout().execute(MoveTo(0, prompt_line));
    let _ = stdout().execute(Clear(ClearType::CurrentLine));
    for line in display_lines {
        let _ = stdout().write_all(line.as_bytes());
    }
    let _ = stdout().execute(MoveTo(cursor_x, prompt_line));
}
