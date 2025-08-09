use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use crate::repl::Repl;


pub(super) fn on_toggle_key_bevy(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut repl: ResMut<Repl>,
) {
    // Map default backtick to Bevy Backquote. Extend this if you support other toggle keys.
    if matches!(repl.toggle_key, Some(KeyCode::Backquote)) {
        if keyboard.just_pressed(KeyCode::Backquote) {
            repl.toggle();
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