use crate::{Repl, ReplSet, repl_enabled};
use bevy::prelude::*;
use std::collections::VecDeque;

pub(crate) struct ReplInputPlugin;

impl Plugin for ReplInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplCommandQueue::default())
            .configure_sets(
                Update,
                ReplSet::Input.run_if(repl_enabled).after(ReplSet::First),
            )
            .add_systems(Update, repl_input_system.in_set(ReplSet::Input));
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

#[derive(Resource, Default)]
pub struct ReplCommandQueue {
    pub commands: VecDeque<String>,
}
