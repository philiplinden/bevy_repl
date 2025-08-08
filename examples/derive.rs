use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use clap::Parser;

// Define a simple command struct using clap's derive pattern
#[derive(Parser, ReplCommand, Debug, Clone, Event)]
#[command(
    name = "say",
    about = "Say something"
)]
struct SayCommand {
    #[arg(help = "Message to say")]
    message: String,
    #[arg(short = 'r', long = "repeat", help = "Number of times to repeat", default_value = "1")]
    repeat: usize,
}

// System that handles the command with access to Bevy ECS
fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    // Print the main message
    info!("Saying: {}", command.message);
    
    // Print repeated messages
    for i in 0..command.repeat {
        info!("{}: {}", i + 1, command.message);
    }
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            ReplPlugins,
        ))
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .run();
}
