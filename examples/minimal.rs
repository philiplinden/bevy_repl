use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        bevy_repl::ReplPlugin,
    ));

    app.run();
}
// configure headless rendering if gui feature is disabled
#[cfg(not(feature = "gui"))]
{
    group = group.add(bevy::app::ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 60.0),
    ));
}