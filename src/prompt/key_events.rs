use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use crate::repl::Repl;
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::crossterm::event::KeyEventKind as CrosstermKeyEventKind;
use crate::prompt::keymap;


pub(super) fn on_toggle_key_bevy(
    // Optional: Bevy ButtonInput may not be populated with MinimalPlugins
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    // Always available via bevy_ratatui
    mut ct_events: EventReader<KeyEvent>,
    mut repl: ResMut<Repl>,
) {
    if let Some(target) = repl.toggle_key {
        // Try Bevy input first if present
        if let Some(kb) = keyboard.as_ref() {
            if kb.just_pressed(target) {
                repl.toggle();
                return;
            }
        }

        // Fall back to Crossterm key events mapped to Bevy KeyCode
        for event in ct_events.read() {
            if event.kind == CrosstermKeyEventKind::Press {
                if let Some(bevy_key) = keymap::crossterm_to_bevy(&event.code) {
                    if bevy_key == target {
                        repl.toggle();
                        return;
                    }
                }
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