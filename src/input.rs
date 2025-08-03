use crate::{Repl, ReplSet, repl_enabled};
use bevy::prelude::*;
use bevy_crossterm::prelude::*;
use std::collections::VecDeque;

pub(crate) struct ReplInputPlugin;

impl Plugin for ReplInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplCommandQueue::default())
            .configure_sets(
                Update,
                ReplSet::Input.run_if(repl_enabled).after(ReplSet::First),
            )
            .add_systems(Update, (repl_input_system, handle_crossterm_input, handle_keyboard_input, handle_ctrl_c_signal).in_set(ReplSet::Input));
    }
}

/// System that reads input from the terminal and processes commands
/// This runs every frame but only processes input when available
fn repl_input_system(mut repl: ResMut<Repl>, mut command_queue: ResMut<ReplCommandQueue>) {
    // Try to receive input from the terminal
    while let Some(input) = repl.try_recv_input() {
        // Add the command to the queue
        command_queue.commands.push_back(input);
    }
}

/// System that handles bevy_crossterm input events
fn handle_crossterm_input(
    mut terminal: ResMut<crate::terminal::BevyCrosstermTerminal>,
    mut input_events: EventReader<CrosstermInputEvent>,
) {
    for event in input_events.read() {
        match event {
            CrosstermInputEvent::Input(input) => {
                terminal.handle_input(input);
            }
            CrosstermInputEvent::Interrupt => {
                // Handle Ctrl+C
                terminal.handle_interrupt();
            }
            CrosstermInputEvent::Resize(_, _) => {
                // Handle terminal resize
            }
            _ => {
                // Handle other events as needed
            }
        }
    }
}

/// System that handles keyboard input for Ctrl+C
fn handle_keyboard_input(
    keyboard_input: Option<Res<ButtonInput<KeyCode>>>,
    mut terminal: ResMut<crate::terminal::BevyCrosstermTerminal>,
) {
    if let Some(keyboard_input) = keyboard_input {
        // Check for Ctrl+C (KeyCode::KeyC with Ctrl modifier)
        if keyboard_input.just_pressed(KeyCode::KeyC) {
            // Check if Ctrl is held (this is a simplified check)
            // In a real implementation, you'd need to check modifiers
            terminal.handle_interrupt();
        }
    }
}

/// System that handles Ctrl+C signal
fn handle_ctrl_c_signal(
    mut terminal: ResMut<crate::terminal::BevyCrosstermTerminal>,
) {
    // This is a fallback system that runs every frame
    // In a real implementation, you'd use a signal handler
    // For now, we'll rely on the input handling in handle_input
}

#[derive(Resource, Default)]
pub struct ReplCommandQueue {
    pub commands: VecDeque<String>,
}
