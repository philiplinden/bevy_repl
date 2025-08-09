//! Minimal Bevy REPL example.
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
    println!("Pong");
}

fn instructions() {
    println!();
    println!("Welcome to the Bevy REPL minimal example!");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    println!("  `quit`    - Close the app.");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                // Run headless in the terminal
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(Startup, instructions)
        .run();
}
