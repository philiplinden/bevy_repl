//! Bevy log routing example.
//!
//! Demonstrates:
//! - Routing Bevy/tracing logs to the REPL
//! - Printing messages directly to the console with `repl_println!`

use bevy::prelude::*;
use bevy_repl::prelude::*;

use std::time::Duration;

fn instructions() {
    bevy_repl::repl_println!("\nBevy log routing example");
    bevy_repl::repl_println!("Tracing logs are printed in the terminal above the prompt");
    bevy_repl::repl_println!("just like a message that was printed directly.");
    bevy_repl::repl_println!("\nType `ping` to emit some logs.");
    bevy_repl::repl_println!("Type `quit` to exit.");
}

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn error_on_ping(_trigger: Trigger<PingCommand>) {
    tracing::error!("Pong");
}

fn warn_on_ping(_trigger: Trigger<PingCommand>) {
    tracing::warn!("Pong");
}

fn info_on_ping(_trigger: Trigger<PingCommand>) {
    tracing::info!("Pong");
}

fn debug_on_ping(_trigger: Trigger<PingCommand>) {
    tracing::debug!("Pong");
}

fn trace_on_ping(_trigger: Trigger<PingCommand>) {
    tracing::trace!("Pong");
}

fn print_on_ping(_trigger: Trigger<PingCommand>) {
    repl_println!("(direct print via repl_println!) Pong");
}

fn main() {
    App::new()
        .add_plugins((
            // Disable stdout logger to avoid duplicate output; our fmt layer prints
            DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(error_on_ping)
        .add_observer(warn_on_ping)
        .add_observer(info_on_ping)
        .add_observer(debug_on_ping)
        .add_observer(trace_on_ping)
        .add_observer(print_on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
