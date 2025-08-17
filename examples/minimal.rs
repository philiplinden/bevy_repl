//! Minimal Bevy REPL example.
//! 
//! It's minimal in the sense that it has the minimum features enabled and the least dependencies.
//!
//! Demonstrates:
//! - Registering a simple `ReplCommand` (ping)
//! - Running headless via `ScheduleRunnerPlugin`
//! - Toggling the REPL with a key (`Repl::toggle_key`)
//! - Typing commands in the terminal and quitting with `quit`
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    bevy::log::info!("Pong");
}

fn instructions() {
    bevy::log::info!("");
    bevy::log::info!("Welcome to the Bevy REPL minimal example!");
    bevy::log::info!("");
    bevy::log::info!("Try typing a command:");
    bevy::log::info!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    bevy::log::info!("");
    bevy::log::info!("Press CTRL+C to exit any time.");
    bevy::log::info!("");
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins
                // Run headless in the terminal
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            // Input plugin is required so the REPL can handle keyboard input
            bevy::input::InputPlugin::default(),
            // Use the minimal renderer with a custom ratatui context that
            // operates in the main terminal instead of an alternate screen
            MinimalReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
