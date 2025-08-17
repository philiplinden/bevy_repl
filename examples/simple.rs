use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

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

// Define a simple command struct
#[derive(Debug, Clone, Event, Default)]
struct SayCommand {
    message: String,
    repeat: usize,
    shout: bool,
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

// System that handles the command with access to Bevy ECS
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

// --- Logging demo: start/stop a periodic log stream with auto-stop ---
#[derive(Resource)]
struct LogStream {
    active: bool,
    tick: Timer,
    auto_stop: Option<Timer>,
}

#[derive(Debug, Clone, Event, Default)]
struct StartCommand;
impl ReplCommand for StartCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("start").about("Start a demo log stream (auto-stops after 5s)")
    }
}

#[derive(Debug, Clone, Event, Default)]
struct StopCommand;
impl ReplCommand for StopCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("stop").about("Stop the demo log stream")
    }
}

fn on_start(_t: Trigger<StartCommand>, mut stream: ResMut<LogStream>) {
    stream.active = true;
    stream.auto_stop = Some(Timer::from_seconds(5.0, TimerMode::Once));
    repl_println!("Log stream started (auto-stop in ~5s). Use `stop` to end early.");
}

fn on_stop(_t: Trigger<StopCommand>, mut stream: ResMut<LogStream>) {
    if stream.active {
        stream.active = false;
        stream.auto_stop = None;
        repl_println!("Log stream stopped.");
    } else {
        repl_println!("Log stream is not running.");
    }
}

fn log_streamer(time: Res<Time>, mut stream: ResMut<LogStream>) {
    if stream.active {
        if stream.tick.tick(time.delta()).just_finished() {
            // Use logging to stdout to demonstrate scrolling above prompt
            bevy::log::info!("tick at {:.2}s", time.elapsed_secs());
        }
        if let Some(timer) = stream.auto_stop.as_mut() {
            if timer.tick(time.delta()).finished() {
                stream.active = false;
                stream.auto_stop = None;
                repl_println!("Auto-stopped log stream.");
            }
        }
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL builder example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`                     - Ping the app");
    repl_println!("  `say <message>`            - Say a message");
    repl_println!("  `say -s <message>`         - Shout the message");
    repl_println!("  `say -r N <message>`       - Repeat N times");
    repl_println!("  `start`                    - Start a log stream");
    repl_println!("  `stop`                     - Stop the log stream");
    repl_println!("  `quit`                     - Close the app");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            ReplPlugins,
        ))
        .insert_resource(LogStream {
            active: false,
            tick: Timer::from_seconds(0.5, TimerMode::Repeating),
            auto_stop: None,
        })
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .add_repl_command::<StartCommand>()
        .add_observer(on_start)
        .add_repl_command::<StopCommand>()
        .add_observer(on_stop)
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .add_systems(Update, log_streamer)
        .run();
}
