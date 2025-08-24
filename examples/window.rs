//! Windowed Bevy REPL example.
//!
//! Demonstrates:
//! - Using the REPL while a Bevy window is open
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    info!("Pong");
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL window example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    repl_println!("  `quit`    - Close the app.");
    repl_println!();
    repl_println!("Press CTRL+C or close the window to exit.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy REPL".to_string(),
                    ..default()
                }),
                ..default()
            }),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
