use crate::{
    Repl, ReplSet, input::ReplCommandQueue, registry::ReplCommandRegistry, repl_enabled,
};
use bevy::prelude::*;

pub(crate) struct ReplExecutionPlugin;

impl Plugin for ReplExecutionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ReplSet::Execution
                .run_if(repl_enabled)
                .after(ReplSet::Input),
        )
        .add_systems(Update, (command_execution_system,).in_set(ReplSet::Execution));
    }
}

/// System that executes commands from the registry
fn command_execution_system(
    mut command_queue: ResMut<ReplCommandQueue>,
    registry: ResMut<ReplCommandRegistry>,
    mut commands: Commands,
    repl: ResMut<Repl>,
) {
    while let Some(command_input) = command_queue.commands.pop_front() {
        match registry.parse_and_execute(&command_input, &mut commands) {
            Ok(output) => {
                repl.send_output(output);
            }
            Err(e) => {
                repl.send_output(format!("Error: {}", e));
            }
        }
    }
}
