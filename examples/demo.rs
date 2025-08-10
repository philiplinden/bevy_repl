//! Pretty REPL prompt example (Override approach).
//!
//! This example shows how to configure the pretty styling by overriding
//! `ReplPromptConfig` and `ReplPrompt` after adding `ReplPlugins`.
//!
//! Run with: `cargo run --example pretty --features pretty`

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::{prelude::*, prompt::PromptPlugin};
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
    println!("Welcome to the Bevy REPL pretty prompt example!");
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
            DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            ReplPlugins.set(PromptPlugin::pretty()),
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
