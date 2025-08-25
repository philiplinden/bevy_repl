//! Bevy log routing example.
//! 
//! Demonstrates:
//! - Routing Bevy/tracing logs to the REPL
//! - Printing messages directly to the console with `repl_println!`

use bevy::prelude::*;
use bevy_repl::prelude::*;

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

fn on_ping(_trigger: Trigger<PingCommand>) {
    tracing::error!("Pong");
    tracing::warn!("Pong");
    tracing::info!("Pong");
    tracing::debug!("Pong");
    tracing::trace!("Pong");
    repl_println!("(direct print via repl_println!) Pong");
}

fn main() {
    // Install a global fmt layer that writes logs directly to the REPL printer,
    // preserving colors/formatting. Do this BEFORE adding DefaultPlugins.
    tracing_to_repl_fmt();

    App::new()
        .add_plugins((
            // Disable stdout logger to avoid duplicate output; our fmt layer prints
            DefaultPlugins.build().disable::<bevy::log::LogPlugin>(),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
