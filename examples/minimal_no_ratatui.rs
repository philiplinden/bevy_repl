//! Minimal REPL example without ratatui integration.
//!
//! This example demonstrates using `MinimalReplPlugins` which includes only
//! the core REPL systems, parser, and prompt plugin - no bevy_ratatui integration
//! and no built-in commands. The prompt will render using the fallback `ReplContext`
//! on the main terminal screen.
//!
//! Run with: `cargo run --example minimal_no_ratatui`

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::{prelude::*, plugin::MinimalReplPlugins};
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

#[derive(Debug, Clone, Event, Default)]
struct ExitCommand;

impl ReplCommand for ExitCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("exit").about("Exit the application")
    }
}

fn on_exit(_trigger: Trigger<ExitCommand>, mut exit: EventWriter<bevy::app::AppExit>) {
    exit.write(bevy::app::AppExit::Success);
}

fn instructions() {
    println!();
    println!("Welcome to the Bevy REPL minimal (no ratatui) example!");
    println!();
    println!("This example uses MinimalReplPlugins which:");
    println!("- Does NOT include bevy_ratatui (no alternate screen)");
    println!("- Does NOT include built-in commands (no help/clear)");
    println!("- Renders the prompt on the main terminal screen");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    println!("  `exit`    - Close the app.");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            bevy::log::LogPlugin::default(),
            // Use MinimalReplPlugins instead of ReplPlugins
            // This excludes bevy_ratatui and built-in commands
            MinimalReplPlugins,
        ))
        // Add our custom commands since built-ins are not included
        .add_repl_command::<PingCommand>()
        .add_repl_command::<ExitCommand>()
        .add_observer(on_ping)
        .add_observer(on_exit)
        .add_systems(Startup, instructions)
        .run();
}
