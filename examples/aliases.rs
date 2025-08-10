//! Aliases example for Bevy REPL using clap.
//!
//! Demonstrates:
//! - Defining a REPL command with multiple aliases via clap
//! - All aliases map to the same command implementation transparently
//!
//! Try typing in the REPL (all do the same thing):
//!   say     <message>
//!   s       <message>
//!   print   <message>
//!   echo    <message>

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

// Define a simple command struct
#[derive(Debug, Clone, Event, Default)]
struct SayCommand {
    message: String,
}

// Implement ReplCommand trait with builder pattern
impl ReplCommand for SayCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
            .alias("s")
            .alias("print")
            .alias("echo")
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let message = matches.get_one::<String>("message").unwrap().clone();
        Ok(SayCommand { message })
    }
}

// System that handles the command with access to Bevy ECS
fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    let message = command.message.clone();
    // Print the main message
    println!("Saying: {}", message);
}

fn instructions() {
    println!();
    println!("Bevy REPL aliases example (clap)");
    println!();
    println!("These are all equivalent:");
    println!("  say     <message>");
    println!("  s       <message>");
    println!("  print   <message>");
    println!("  echo    <message>");
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            bevy::input::InputPlugin::default(),
            ReplPlugins,
        ))
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .add_systems(PostStartup, instructions)
        .run();
}
