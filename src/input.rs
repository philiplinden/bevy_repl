use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::keymap::ReplKeymap;
use crate::repl::{Repl, ReplBufferEvent, ReplLifecycleEvent, ReplSet, ReplSubmitEvent};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                capture_terminal_input
                    .in_set(ReplSet::Capture)
                    .in_set(ReplSet::All),
                update_repl_buffer
                    .in_set(ReplSet::Buffer)
                    .in_set(ReplSet::All),
                suppress_game_keyboard_input
                    .in_set(ReplSet::Post)
                    .in_set(ReplSet::All),
            ),
        );
    }
}

/// System that captures keyboard input from the terminal and emits events to the REPL buffer.
pub fn capture_terminal_input(
    mut buffer_events: MessageWriter<ReplBufferEvent>,
    mut lifecycle_events: MessageWriter<ReplLifecycleEvent>,
    repl: Res<Repl>,
    keymap: Res<ReplKeymap>,
) {
    if !repl.enabled {
        return;
    }

    while event::poll(Duration::ZERO).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                // Check for lifecycle transitions (toggle, enable, disable)
                if let Some(lifecycle_ev) = keymap.check_lifecycle(&key_event) {
                    lifecycle_events.write(lifecycle_ev);
                    continue;
                }

                // Map key event to buffer action
                if let Some(buf_ev) = keymap.map(&key_event) {
                    buffer_events.write(buf_ev);
                }
            }
        }
    }
}

/// System that updates the REPL buffer with events from the keymap and triggers submission observers.
fn update_repl_buffer(
    mut repl: ResMut<Repl>,
    mut buffer_events: MessageReader<ReplBufferEvent>,
    mut commands: Commands,
) {
    for event in buffer_events.read() {
        match event {
            ReplBufferEvent::Insert(c) => repl.insert(*c),
            ReplBufferEvent::Backspace => repl.backspace(),
            ReplBufferEvent::Delete => repl.delete(),
            ReplBufferEvent::MoveLeft => repl.left(),
            ReplBufferEvent::MoveRight => repl.right(),
            ReplBufferEvent::JumpToStart => repl.home(),
            ReplBufferEvent::JumpToEnd => repl.end(),
            ReplBufferEvent::Clear => repl.clear_buffer(),
            ReplBufferEvent::ClearToStart => repl.clear_to_start(),
            ReplBufferEvent::ClearScreen => {
                let _ = Repl::clear_terminal();
            }
            ReplBufferEvent::Submit => {
                let input = repl.drain_buffer();
                if !input.is_empty() {
                    commands.trigger(ReplSubmitEvent(input));
                }
            }
        }
    }
}

/// Blocks keyboard input from being forwarded to Bevy when REPL is enabled to
/// prevent key events from reaching game systems while typing into the prompt.
fn suppress_game_keyboard_input(
    mut key_events: ResMut<Messages<KeyboardInput>>,
    mut keyboard_input: ResMut<ButtonInput<bevy::input::keyboard::KeyCode>>,
    repl: Res<Repl>,
) {
    if repl.enabled {
        key_events.clear();
        keyboard_input.reset_all();
    }
}
