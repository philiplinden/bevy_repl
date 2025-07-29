use crate::{Repl, ReplSet, repl_enabled};
use bevy::prelude::*;

pub(crate) struct ReplOutputPlugin;

impl Plugin for ReplOutputPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PrintReplLine>()
        .configure_sets(
            Update,
            ReplSet::Output
                .run_if(repl_enabled)
                .after(ReplSet::Execution),
        )
        .add_systems(Update, repl_output_system.in_set(ReplSet::Output));
    }
}

/// Event that can be sent to print a line to the REPL
#[derive(Event)]
pub struct PrintReplLine(pub String);

/// System that handles REPL output
/// This ensures output is printed in a thread-safe way
fn repl_output_system(mut print_events: EventReader<PrintReplLine>, repl: Res<Repl>) {
    for event in print_events.read() {
        repl.send_output(event.0.clone());
    }
}
