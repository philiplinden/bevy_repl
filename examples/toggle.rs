//! REPL toggle-only example.
//!
//! Demonstrates:
//! - Toggling the REPL with a key (`Repl::toggle_key`)
//! - Running a simple command while REPL is enabled
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

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(Startup, instructions)
        .run();
}

fn instructions() {
    println!();
    println!("Bevy REPL toggle example (simple)");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command (prints Pong)");
    println!("  `quit`    - Close the app");
    println!();
    println!("The REPL can be toggled with:");
    println!("  {:?}", Repl::default().toggle_key.unwrap());
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}