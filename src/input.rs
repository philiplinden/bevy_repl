use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crossterm::event::{self, Event, KeyEventKind};
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
                suppress_game_keyboard_input
                    .in_set(ReplSet::Post)
                    .in_set(ReplSet::All),
            ),
        );
    }
}

/// System that captures keyboard input from the terminal and immediately updates the REPL buffer.
pub fn capture_terminal_input(
    mut repl: ResMut<Repl>,
    mut lifecycle_events: MessageWriter<ReplLifecycleEvent>,
    mut app_exit: MessageWriter<AppExit>,
    mut commands: Commands,
    keymap: Res<ReplKeymap>,
) {
    if !repl.enabled {
        return;
    }

    while event::poll(Duration::ZERO).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                // Ctrl+C in raw mode: immediately restore terminal and exit
                if key_event
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    && key_event.code == crossterm::event::KeyCode::Char('c')
                {
                    let _ = Repl::restore_terminal();
                    app_exit.write(AppExit::Success);
                    return;
                }

                // 1. Check for lifecycle transitions (toggle, enable, disable)
                if let Some(lifecycle_ev) = keymap.check_lifecycle(&key_event) {
                    lifecycle_events.write(lifecycle_ev);
                    continue;
                }

                // 2. Map key event to buffer action and apply immediately
                if let Some(action) = keymap.map(&key_event) {
                    match action {
                        ReplBufferEvent::Insert(c) => repl.insert(c),
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
