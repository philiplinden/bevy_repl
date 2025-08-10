//! Derive-based command example for Bevy REPL.
//!
//! Demonstrates:
//! - Defining a command with clap's derive macros
//! - Automatic `ReplCommand` via `#[derive(ReplCommand)]`
//! - Parsing args/flags into a typed struct
use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use clap::Parser;

// Define a simple command struct using clap's derive pattern
#[derive(Parser, ReplCommand, Debug, Clone, Event, Default)]
#[command(
    name = "say",
    about = "Say something"
)]
struct SayCommand {
    #[arg(help = "Message to say")]
    message: String,
    #[arg(short = 'r', long = "repeat", help = "Number of times to repeat", default_value = "1")]
    repeat: usize,
    #[arg(short = 's', long = "shout", help = "Shout the message", action = clap::ArgAction::SetTrue, num_args = 0)]
    shout: bool,
}

// System that handles the command with access to Bevy ECS
fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    let message = if command.shout {
        command.message.to_uppercase()
    } else {
        command.message.clone()
    };
    // Print the main message
    println!("Saying: {}", message);
    
    // Print repeated messages
    for i in 0..command.repeat {
        println!("{}: {}", i + 1, message);
    }
}

fn instructions() {
    println!();
    println!("Welcome to the Bevy REPL derive example!");
    println!();
    println!("Try typing a command:");
    println!("  `say <message>`            - Say a message");
    println!("  `say -s <message>`         - Shout the message");
    println!("  `say -r N <message>`       - Repeat N times");
    println!("  `quit`                     - Close the app");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            ReplPlugins,
        ))
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .add_systems(PostStartup, instructions)
        .run();
}
