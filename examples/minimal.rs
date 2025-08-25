//! Minimal Bevy REPL example.
//! 
//! This example shows the minimum amount of dependencies needed to use the REPL.
//! 
//! Demonstrates:
//! - Registering a simple `ReplCommand` (ping)
//! - Running headless via `ScheduleRunnerPlugin`
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
    repl_println!("Pong");
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL stdout example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
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
            // Runs the REPL headless in the terminal
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
