//! Pretty REPL prompt example (custom style).
//!
//! This example shows configuring the pretty styling inserting a custom
//! `ReplPromptConfig` resource.

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::prelude::Stylize;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    bevy::log::info!("Pong");
}

#[derive(Debug, Clone, Event, Default)]
struct SayCommand {
    message: String,
    repeat: usize,
    shout: bool,
}

impl ReplCommand for SayCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
            .alias("echo")
            .arg(
                clap::Arg::new("repeat")
                    .short('r')
                    .long("repeat")
                    .help("Number of times to repeat")
                    .default_value("1"),
            )
            .arg(
                clap::Arg::new("shout")
                    .short('s')
                    .long("shout")
                    .help("Shout the message")
                    .action(clap::ArgAction::SetTrue)
                    .num_args(0),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let message = matches.get_one::<String>("message").unwrap().clone();
        let repeat = matches
            .get_one::<String>("repeat")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        let shout = matches.get_flag("shout");

        Ok(SayCommand {
            message,
            repeat,
            shout,
        })
    }
}

fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    let message = if command.shout {
        command.message.to_uppercase()
    } else {
        command.message.clone()
    };
    // Print the main message
    repl_println!("Saying: {}", message);

    // Print repeated messages
    for i in 0..command.repeat {
        repl_println!("{}: {}", i + 1, message);
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL pretty example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`                     - Ping the app");
    repl_println!("  `say <message>`            - Say a message");
    repl_println!("  `say -s <message>`         - Shout the message");
    repl_println!("  `say -r N <message>`       - Repeat N times");
    repl_println!("  `quit`                     - Close the app");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
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
        // Override the prompt config by inserting the resource here
        .insert_resource(ReplPromptConfig {
            symbol: Some(PromptSymbol {
                text: "λ ".to_string(),
                style: Style::default().fg(Color::Cyan).bold(),
            }),
            block: Some(Block::default().borders(Borders::ALL).title("REPL")),
            hint: Some(PromptHint {
                text: "Enter to run • Esc to clear".to_string(),
                style: Style::default().fg(Color::Gray).italic(),
            }),
        })
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
