use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    // Minimal pretty logging example placeholder so tests/builds succeed.
    let mut app = App::new();
    app.add_plugins(MinimalReplPlugins);

    tracing::info!("log_pretty example initialized");

    app.update();
}
