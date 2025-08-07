use bevy::prelude::*;
use bevy_ratatui::{
    crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind},
    crossterm::{
        ExecutableCommand,
        cursor::MoveTo,
        terminal::{self, Clear, ClearType},
    },
    event::{InputSet, KeyEvent},
};
use std::io::{Write, stdout};

use crate::repl::{Repl, repl_is_enabled};

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
            buffer: String::new(),
        });
        app.add_event::<ReplBufferEvent>();
        app.add_event::<ReplSubmitEvent>();
        app.add_systems(
            Update,
            (capture_repl_input, update_repl_buffer)
                .chain()
                .before(InputSet::EmitBevy)
                .run_if(repl_is_enabled),
        );
        app.add_systems(
            Update,
            block_event_forwarding
                .in_set(InputSet::EmitBevy)
                .run_if(repl_is_enabled),
        );
        app.add_systems(
            Update,
            display_prompt
                .run_if(repl_is_enabled)
                .run_if(resource_changed::<Repl>)
                .in_set(InputSet::Post),
        );
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub buffer: String,
}

#[derive(Event)]
pub struct ReplSubmitEvent(pub String);

#[derive(Event)]
pub enum ReplBufferEvent {
    Insert(char),
    Backspace,
    Delete,
    MoveLeft,
    MoveRight,
    JumpToStart,
    JumpToEnd,
    Clear,
    Submit,
}

fn capture_repl_input(
    mut crossterm_key_events: EventReader<KeyEvent>,
    mut buffer_events: EventWriter<ReplBufferEvent>,
    repl: Res<Repl>,
) {
    for event in crossterm_key_events.read() {
        if event.kind == CrosstermKeyEventKind::Press {
            // Skip processing if this is the toggle key
            if let Some(toggle_key) = repl.toggle_key {
                if event.code == toggle_key {
                    continue; // Consume the toggle key without processing it
                }
            }

            match event.code {
                CrosstermKeyCode::Enter => {
                    buffer_events.write(ReplBufferEvent::Submit);
                }
                CrosstermKeyCode::Char(c) => {
                    buffer_events.write(ReplBufferEvent::Insert(c));
                }
                CrosstermKeyCode::Backspace => {
                    buffer_events.write(ReplBufferEvent::Backspace);
                }
                CrosstermKeyCode::Left => {
                    buffer_events.write(ReplBufferEvent::MoveLeft);
                }
                CrosstermKeyCode::Right => {
                    buffer_events.write(ReplBufferEvent::MoveRight);
                }
                CrosstermKeyCode::Home => {
                    buffer_events.write(ReplBufferEvent::JumpToStart);
                }
                CrosstermKeyCode::End => {
                    buffer_events.write(ReplBufferEvent::JumpToEnd);
                }
                CrosstermKeyCode::Delete => {
                    buffer_events.write(ReplBufferEvent::Delete);
                }
                CrosstermKeyCode::Esc => {
                    buffer_events.write(ReplBufferEvent::Clear);
                }
                _ => { /* ignore other non-character keys */ }
            }
        }
    }
}

fn update_repl_buffer(
    mut repl: ResMut<Repl>,
    mut buffer_events: EventReader<ReplBufferEvent>,
    mut parse_events: EventWriter<ReplSubmitEvent>,
) {
    for event in buffer_events.read() {
        match event {
            ReplBufferEvent::Insert(c) => {
                repl.insert(*c);
            }
            ReplBufferEvent::Backspace => {
                repl.backspace();
            }
            ReplBufferEvent::Delete => {
                repl.delete();
            }
            ReplBufferEvent::MoveLeft => {
                repl.left();
            }
            ReplBufferEvent::MoveRight => {
                repl.right();
            }
            ReplBufferEvent::JumpToStart => {
                repl.home();
            }
            ReplBufferEvent::JumpToEnd => {
                repl.end();
            }
            ReplBufferEvent::Clear => {
                repl.clear_buffer();
            }
            ReplBufferEvent::Submit => {
                let input = repl.drain_buffer();
                // Print a newline to move terminal to next line
                let _ = stdout().write_all(b"\n");
                parse_events.write(ReplSubmitEvent(input));
            }
        }
    }
}

/// System that displays the current input buffer at the bottom of the terminal
/// Runs whenever the Repl resource changes
fn display_prompt(repl: Res<Repl>, prompt: Res<ReplPrompt>) {
    // Get terminal size
    let (width, height) = match terminal::size() {
        Ok(size) => size,
        Err(_) => return, // If we can't get terminal size, skip rendering
    };

    // Display the prompt and current buffer
    let prompt_text = if let Some(symbol) = prompt.symbol.clone() {
        format!("{}{}", symbol, repl.buffer)
    } else {
        repl.buffer.clone()
    };

    // Position cursor at the correct location within the buffer
    let prompt_symbol_len = prompt.symbol.as_ref().map(|s| s.len()).unwrap_or(0);
    let cursor_x = (prompt_symbol_len + repl.cursor_pos) as u16;
    let cursor_x = cursor_x.min(width.saturating_sub(1));

    // Execute terminal operations sequentially to avoid borrow checker issues
    let _ = stdout().execute(MoveTo(0, height.saturating_sub(1)));
    let _ = stdout().execute(Clear(ClearType::CurrentLine));
    let _ = stdout().write_all(prompt_text.as_bytes());
    let _ = stdout().execute(MoveTo(cursor_x, height.saturating_sub(1)));
}

/// System that blocks event forwarding to Bevy when REPL is enabled
/// This prevents key events from reaching game systems during REPL input.
/// The toggle key is always allowed to pass through for REPL toggling.
fn block_event_forwarding(mut key_events: EventReader<KeyEvent>, repl: Res<Repl>) {
    // Read all events once and filter out the toggle key
    let events: Vec<_> = key_events.read().collect();

    for event in events {
        // Allow toggle key to pass through
        if let Some(toggle_key) = repl.toggle_key {
            if event.code == toggle_key {
                continue; // Skip blocking this event
            }
        }
        // All other events are consumed (blocked) when REPL is enabled
    }
}
