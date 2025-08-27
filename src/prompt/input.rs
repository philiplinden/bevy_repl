use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy_ratatui::crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind};
use bevy_ratatui::event::KeyEvent;
use std::io::{stdout, Write};

use crate::repl::{Repl, ReplBufferEvent, ReplSubmitEvent, ReplSet};

pub struct PromptInputPlugin;

impl Plugin for PromptInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // When enabled, capture terminal input
                capture_repl_input
                    .in_set(ReplSet::Capture)
                    .in_set(ReplSet::All),
                // Then update the REPL buffer explicitly after capture
                update_repl_buffer
                    .in_set(ReplSet::Buffer)
                    .in_set(ReplSet::All)
                    .after(ReplSet::Capture),
                // Block keyboard input from being forwarded to Bevy when REPL is enabled to
                // prevent key events from reaching game systems while typing into the prompt.
                block_keyboard_input_forwarding
                    .in_set(ReplSet::Post)
                    .in_set(ReplSet::All)
                    .after(ReplSet::Render),
            ),
        );
    }
}

/// System that blocks keyboard input from being forwarded to Bevy when REPL is enabled to
/// prevent key events from reaching game systems while typing into the prompt.
pub(super) fn block_keyboard_input_forwarding(
    mut key_events: ResMut<Events<KeyboardInput>>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
) {
    // Clear all keyboard events
    key_events.clear();
    keyboard_input.reset_all();
}

fn capture_repl_input(
    mut crossterm_key_events: EventReader<KeyEvent>,
    mut buffer_events: EventWriter<ReplBufferEvent>,
) {
    for event in crossterm_key_events.read() {
        if event.kind == CrosstermKeyEventKind::Press {
            match event.code {
                CrosstermKeyCode::Char(c) => {
                    // Optional: treat control-altered chars differently
                    // use bevy_ratatui::crossterm::event::KeyModifiers;
                    if event.modifiers.is_empty() {
                        // Only emit Insert event if no modifiers are pressed
                        buffer_events.write(ReplBufferEvent::Insert(c));
                    }
                }
                CrosstermKeyCode::Enter => {
                    buffer_events.write(ReplBufferEvent::Submit);
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


/// System that updates the REPL buffer with events from the prompt. This is
/// separate from the system that directly handles key events to allow for
/// custom keybinds.
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
                let _ = stdout().write_all(b"\r");
                parse_events.write(ReplSubmitEvent(input));
            }
        }
    }
}
