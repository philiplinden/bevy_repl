//! Pretty REPL prompt example (Manual composition).
//!
//! This example shows configuring the pretty styling by composing the
//! plugins manually and passing a custom `PromptPlugin` with a
//! `ReplPromptConfig` preset.
//!
//! Run with: `cargo run --example pretty_manual --features pretty`

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::{
    prelude::*,
    prompt::{
        PromptBorderConfig, PromptColorConfig, PromptHintConfig, renderer::pretty::PrettyRenderer,
    },
};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    println!("Pong");
}

fn instructions() {
    println!();
    println!("Welcome to the Bevy REPL pretty prompt example (manual)!");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command. (it outputs Pong)");
    println!("  `quit`    - Close the app.");
    println!();
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
            ReplPlugins.set(PromptPlugin {
                config: ReplPromptConfig {
                    symbol: Some("λ ".into()),
                    border: Some(PromptBorderConfig::default()),
                    color: Some(PromptColorConfig::default()),
                    hint: Some(PromptHintConfig::default()),
                },
                renderer: Arc::new(PrettyRenderer),
            }),
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
