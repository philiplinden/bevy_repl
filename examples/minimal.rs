use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Add REPL with custom commands
    app.add_plugins(ReplPlugin)
        .repl::<HelloCommand>(on_hello);

    app.run();
}

#[derive(Parser)]
#[command(name = "hello", about = "Say hello")]
struct HelloCommand;

/// Observer function for hello
fn on_hello(trigger: Trigger<HelloCommand>) {
    info!("Hello, world!");
}
