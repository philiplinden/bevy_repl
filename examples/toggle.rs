//! Example showing how to add the REPL to the app but disable it.
//! All logs and messages should appear as if the REPL were not present.

use bevy::prelude::*;
use bevy_repl::prelude::*;

fn toggle_repl(input: Res<ButtonInput<KeyCode>>, mut repl: ResMut<Repl>) {
    if input.just_pressed(KeyCode::Backquote) {
        repl.enabled = !repl.enabled;
    }
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )),
        ReplPlugins,
    ))
    .add_systems(Update, toggle_repl.in_set(ReplSet::Pre))
    .run();
}
