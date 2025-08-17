//! REPL feature demo
//!
//! For this demo, we enabled the following feature flags:
//! - `quit` to enable the `quit` command (included with `default_commands` feature)
//! - `bevy/bevy_log` to enable logging to stdout with the LogPlugin
//! - `bevy_ratatui/crossterm` for the crossterm TUI backend

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

struct ReplSandboxPlugin;

impl Plugin for ReplSandboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogStream {
            active: false,
            tick: Timer::from_seconds(0.5, TimerMode::Repeating),
            auto_stop: None,
        })
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        // Logging demo
        .add_repl_command::<StartCommand>()
        .add_observer(on_start)
        .add_repl_command::<StopCommand>()
        .add_observer(on_stop)
        .add_systems(Update, log_streamer)
        // Events demo
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        // ECS demo commands
        .add_repl_command::<SpawnCommand>()
        .add_observer(on_spawn)
        .add_repl_command::<ListCommand>()
        .add_observer(on_list)
        .add_repl_command::<QueryCommand>()
        .add_observer(on_query)
        .add_repl_command::<RemoveCommand>()
        .add_observer(on_remove)
        .add_repl_command::<TimeCommand>()
        .add_observer(on_time);
    }
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

// --- Events demo: simple ping command ---
#[derive(Debug, Clone, Event, Default)]
struct PingCommand;
impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Send a ping event; the observer replies with pong")
    }
}

fn on_ping(_t: Trigger<PingCommand>) {
    repl_println!("pong");
}

// --- ECS demo commands ---
#[derive(Debug, Clone, Event, Default)]
struct SpawnCommand {
    name: String,
}
impl ReplCommand for SpawnCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("spawn")
            .about("Spawn an entity with a Name component")
            .arg(
                clap::Arg::new("name")
                    .help("Name for the entity")
                    .required(true),
            )
    }
    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let name = matches.get_one::<String>("name").unwrap().clone();
        Ok(SpawnCommand { name })
    }
}
fn on_spawn(trigger: Trigger<SpawnCommand>, mut commands: Commands) {
    let name = trigger.event().name.clone();
    commands.spawn(Name::new(name.clone()));
    repl_println!("Spawned entity '{name}'.");
}

#[derive(Debug, Clone, Event, Default)]
struct ListCommand;
impl ReplCommand for ListCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("list").about("List all entities with a Name component")
    }
}
fn on_list(_t: Trigger<ListCommand>, query: Query<(Entity, &Name)>) {
    let count = query.iter().count();
    repl_println!("Entities: {}", count);
    for (e, name) in query.iter() {
        repl_println!("- {:?}: {}", e, name.as_str());
    }
}

#[derive(Debug, Clone, Event, Default)]
struct QueryCommand {
    substring: String,
}
impl ReplCommand for QueryCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("query")
            .about("List entities whose Name contains the substring")
            .arg(
                clap::Arg::new("substring")
                    .required(true)
                    .help("Substring to search for"),
            )
    }
    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let substring = matches.get_one::<String>("substring").unwrap().clone();
        Ok(QueryCommand { substring })
    }
}
fn on_query(trigger: Trigger<QueryCommand>, query: Query<(Entity, &Name)>) {
    let sub = trigger.event().substring.to_lowercase();
    let mut found = 0usize;
    for (e, name) in query.iter() {
        if name.as_str().to_lowercase().contains(&sub) {
            repl_println!("({})  {:?}: {}", found + 1, e, name.as_str());
            found += 1;
        }
    }
    repl_println!("Matches: {}", found);
}

#[derive(Debug, Clone, Event, Default)]
struct RemoveCommand {
    substring: String,
}
impl ReplCommand for RemoveCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("remove")
            .about("Remove entities whose Name contains the substring")
            .arg(
                clap::Arg::new("substring")
                    .required(true)
                    .help("Substring filter"),
            )
    }
    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let substring = matches.get_one::<String>("substring").unwrap().clone();
        Ok(RemoveCommand { substring })
    }
}
fn on_remove(
    trigger: Trigger<RemoveCommand>,
    mut commands: Commands,
    query: Query<(Entity, &Name)>,
) {
    let sub = trigger.event().substring.to_lowercase();
    let mut removed = 0usize;
    for (e, name) in query.iter() {
        if name.as_str().to_lowercase().contains(&sub) {
            commands.entity(e).despawn();
            removed += 1;
        }
    }
    repl_println!("Removed: {}", removed);
}

#[derive(Debug, Clone, Event, Default)]
struct TimeCommand;
impl ReplCommand for TimeCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("time").about("Show the current time (since startup)")
    }
}
fn on_time(_t: Trigger<TimeCommand>, time: Res<Time>) {
    repl_println!("elapsed: {:.3}s", time.elapsed_secs());
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL sandbox!");
    repl_println!();
    repl_println!("The following commands are available:");
    repl_println!("  `say <message>`            - Say a message");
    repl_println!("  `say -s <message>`         - Shout the message");
    repl_println!("  `say -r N <message>`       - Repeat N times");
    repl_println!("  `start`                    - Start a demo log stream");
    repl_println!("  `stop`                     - Stop the demo log stream");
    repl_println!("  `ping`                     - Send a ping event");
    repl_println!("  `spawn <name>`             - Spawn an entity with a Name component");
    repl_println!("  `list`                     - List all entities with a Name component");
    repl_println!("  `query <substring>`        - List entities whose Name contains the substring");
    repl_println!("  `remove <substring>`       - Remove entities whose Name contains the substring");
    repl_println!("  `time`                     - Show the current time (since startup)");
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
            ReplSandboxPlugin,
        ))
        .add_systems(PostStartup, instructions)
        .run();
}
