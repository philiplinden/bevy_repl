use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    // Minimal app that installs the REPL and a custom tracing layer, then exits immediately.
    let mut app = App::new();
    app.add_plugins(MinimalReplPlugins);

    // Install a custom tracing layer that forwards to REPL formatting for demonstration.
    tracing_subscriber::registry()
        .with(bevy_repl::prelude::repl_log_custom_layer())
        .init();

    // Emit one log to verify pipeline compiles.
    tracing::info!("log_custom_layer example initialized");

    // Run a single update to exercise startup, then exit.
    app.update();
}
