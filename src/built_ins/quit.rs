use bevy::prelude::*;
use clap::Parser;
use crate::ReplCommand;

/// Quit/Exit command - graceful shutdown
#[derive(Parser, ReplCommand)]
#[command(name = "quit", about = "Gracefully terminate the application", aliases = ["exit", "q"])]
pub struct QuitCommand {
    #[arg(short, long)]
    verbose: bool,
}

/// Observer function for the quit command
pub fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    if trigger.verbose {
        info!("Quitting...");
    }
    exit.send(AppExit::Success);
}
