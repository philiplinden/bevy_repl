use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

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