//! Default Bevy REPL example.
//!
//! This is the minimal code required to use the REPL with example commands that
//! generate logs or print directly to the console using the builder pattern.
//!
//! Demonstrates:
//! - Registering a simple `ReplCommand` (ping)
//! - Running headless via `ScheduleRunnerPlugin`
//! - Typing commands in the terminal and quitting with `quit`

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

// Define a simple command struct
#[derive(Debug, Clone, Event, Default)]
struct PrintCommand {
    message: String,
}

impl ReplCommand for PrintCommand {
    // Define the clap command for this event
    fn clap_command() -> clap::Command {
        clap::Command::new("print")
            .about("Print a message to the console")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
    }

    // Convert clap argument matches to event fields
    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let message = matches.get_one::<String>("message").unwrap().clone();
        Ok(PrintCommand { message })
    }
}

// System that handles the command with access to Bevy ECS
fn on_print(trigger: Trigger<PrintCommand>) {
    let command = trigger.event();
    let message = command.message.clone();
    repl_println!("printing: {}", message);
}

// Define a simple command struct
#[derive(Debug, Clone, Event, Default)]
struct LogCommand {
    message: String,
}

impl ReplCommand for LogCommand {
    // Define the clap command for this event
    fn clap_command() -> clap::Command {
        clap::Command::new("log")
            .about("Emit an info level log message")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
    }

    // Convert clap argument matches to event fields
    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let message = matches.get_one::<String>("message").unwrap().clone();
        Ok(LogCommand { message })
    }
}

// System that handles the command with access to Bevy ECS
fn on_log(trigger: Trigger<LogCommand>) {
    let command = trigger.event();
    let message = command.message.clone();
    tracing::info!("logging: {}", message);
}

fn instructions() {
    repl_println!();
    repl_println!("Bevy REPL default example");
    repl_println!();
    repl_println!("Try typing in the REPL:");
    repl_println!("  print <message>  - Print a message to the console");
    repl_println!("  log <message>    - Emit a log message (info level)");
    repl_println!("  quit             - Exit the App");
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                // Headless loop in the terminal
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            ReplPlugins,
        ))
        .add_repl_command::<PrintCommand>()
        .add_observer(on_print)
        .add_repl_command::<LogCommand>()
        .add_observer(on_log)
        .add_systems(PostStartup, instructions)
        .run();
}
