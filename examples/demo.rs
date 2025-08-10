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
    repl_println!("Pong");
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL pretty prompt example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    repl_println!("  `quit`    - Close the app.");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
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
        .add_systems(PostStartup, instructions.after(ScrollRegionReadySet))
        .run();
}
