use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy_ratatui::crossterm::event::KeyEventKind as CrosstermKeyEventKind;
use bevy_ratatui::event::KeyEvent;
use std::io::{stdout, Write};

use crate::repl::{Repl, ReplBufferEvent, ReplSubmitEvent, ReplSet};
use crate::prompt::keymap::PromptKeymap;

pub struct PromptInputPlugin;

impl Plugin for PromptInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Capture key events from the terminal
                parse_terminal_input
                    .in_set(ReplSet::Capture)
                    .in_set(ReplSet::All),
                // Then update the REPL buffer explicitly after capture
                update_repl_buffer
                    .in_set(ReplSet::Buffer)
                    .in_set(ReplSet::All),
                // Block keyboard input from being forwarded to Bevy when REPL is enabled to
                // prevent key events from reaching game systems while typing into the prompt.
                block_keyboard_input_forwarding
                    .in_set(ReplSet::Post)
                    .in_set(ReplSet::All)
            ),
        );
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

/// System that captures keyboard input from the terminal and emits events to
/// the REPL buffer. This is separate from the system that directly handles key
/// events to allow for custom keybinds for REPL cursor controls someday.
///
/// FIXME: This system does NOT honor modifier keys or chords, so shift-altered
/// keys don't show up as capitals. Only the alphanumeric character is processed
/// and stored to the REPL buffer. Ctrl+C is an exception because it is
/// explicitly handled with the `ctrlc` crate in
/// [`crate::repl::install_terminal_safety_nets`].
pub(super) fn parse_terminal_input(
    mut crossterm_key_events: EventReader<KeyEvent>,
    mut buffer_events: EventWriter<ReplBufferEvent>,
    keymap: Res<PromptKeymap>,
) {
    for event in crossterm_key_events.read() {
        if event.kind == CrosstermKeyEventKind::Press {
            // Parse REPL keybinds
            if let Some(buf_ev) = keymap.map(event) {
                buffer_events.write(buf_ev);
                continue;
            }
            // No binding matched and fallback insert not allowed -> ignore
        }
    }
}
