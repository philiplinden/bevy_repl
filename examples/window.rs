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
    println!();
    println!("Welcome to the Bevy REPL window example!");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    println!("  `quit`    - Close the app.");
    println!();
    println!("The REPL can be toggled with:");
    println!("  {:?}", Repl::default().toggle_key.unwrap());
    println!();
    println!("You can also close the window to exit the app.");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
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
            ReplPlugins,
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(Startup, instructions)
        .run();
}
