use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            // Minimal plugins, headless mode
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            ReplPlugins,
        ))
        .run();
}
