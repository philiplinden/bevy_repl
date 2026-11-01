//! Bevy log routing example.
//!
//! Demonstrates:
//! - Routing Bevy/tracing logs to the REPL
//! - Printing messages directly to the console with `repl_println!`

use bevy::log::{debug, error, info, trace, warn};
use bevy::prelude::*;
use bevy_repl::prelude::*;

fn instructions() {
    repl_println!("\nBevy log routing example");
    repl_println!();
    repl_println!("Tracing logs are printed in the terminal above the prompt");
    repl_println!("just like a message that was printed directly.");
    repl_println!();
    repl_println!("\nType `ping` to emit some logs.");
    repl_println!("Type `quit` to exit.");
}

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }

    fn to_event(_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self)
    }
}

fn error_on_ping(_trigger: On<PingCommand>) {
    error!("Pong");
}

fn warn_on_ping(_trigger: On<PingCommand>) {
    warn!("Pong");
}

fn info_on_ping(_trigger: On<PingCommand>) {
    info!("Pong");
}

fn debug_on_ping(_trigger: On<PingCommand>) {
    debug!("Pong");
}

fn trace_on_ping(_trigger: On<PingCommand>) {
    trace!("Pong");
}

fn print_on_ping(_trigger: On<PingCommand>) {
    repl_println!("(direct print via repl_println!) Pong");
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_secs_f64(1.0 / 60.0),
                ))
                .adapt_for_repl(),
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
