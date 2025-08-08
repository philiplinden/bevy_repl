use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping")
            .about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    info!("Pong");
}

fn main() {
    App::new()
        .add_plugins((
            // Minimal plugins, headless mode
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .run();
}
