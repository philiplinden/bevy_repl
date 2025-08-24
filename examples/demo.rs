//! REPL feature demo
//!
//! Interactive walkthrough of the main features of the REPL
//!
//! For this demo, we enabled the following feature flags:
//! - `quit` to enable the `quit` command (included with `default_commands` feature)
//! - `pretty` to use a fancy style prompt (`PromptPlugin::pretty()`)
//! - `bevy/bevy_log` to enable logging to stdout with the LogPlugin
//! - `bevy_ratatui/crossterm` for the crossterm TUI backend

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

fn repl_print_block(block: &str) {
    for line in block.lines() {
        bevy_repl::repl_println!("{}", line);
    }
}

#[derive(Debug, Clone, Event, Default)]
struct NextCommand;

impl ReplCommand for NextCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("next").about("Advance the demo to the next step")
    }
}

#[derive(Debug, Clone, Event, Default)]
struct BackCommand;

impl ReplCommand for BackCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("back").about("Go back to the previous demo step")
    }
}

fn on_next(_trigger: Trigger<NextCommand>, mut state: ResMut<DemoState>) {
    state.step = match state.step {
        DemoStep::Intro => DemoStep::CommandsAndAliases,
        DemoStep::CommandsAndAliases => DemoStep::Logging,
        DemoStep::Logging => DemoStep::Events,
        DemoStep::Events => DemoStep::Ecs,
        DemoStep::Ecs => DemoStep::Colors,
        DemoStep::Colors => DemoStep::End,
        DemoStep::End => DemoStep::Intro,
    };
}

fn on_back(_trigger: Trigger<BackCommand>, mut state: ResMut<DemoState>) {
    state.step = match state.step {
        DemoStep::Intro => DemoStep::End,
        DemoStep::CommandsAndAliases => DemoStep::Intro,
        DemoStep::Logging => DemoStep::CommandsAndAliases,
        DemoStep::Events => DemoStep::Logging,
        DemoStep::Ecs => DemoStep::Events,
        DemoStep::Colors => DemoStep::Ecs,
        DemoStep::End => DemoStep::Colors,
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DemoStep {
    Intro,
    CommandsAndAliases,
    Logging,
    Events,
    Ecs,
    Colors,
    End,
}

#[derive(Resource)]
struct DemoState {
    step: DemoStep,
    last_printed: Option<DemoStep>,
}

struct DemoPlugin;
impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DemoState {
            step: DemoStep::Intro,
            last_printed: None,
        })
        .insert_resource(LogStream {
            active: false,
            tick: Timer::from_seconds(0.5, TimerMode::Repeating),
            auto_stop: None,
        })
        .add_repl_command::<NextCommand>()
        .add_observer(on_next)
        .add_repl_command::<BackCommand>()
        .add_observer(on_back)
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
        .add_observer(on_time)
        .add_systems(Update, run_step_instructions);
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
            bevy::log::info!("demo log tick at {:.2}s", time.elapsed_secs());
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

// Emit per-step instructions when the step changes
fn run_step_instructions(mut state: ResMut<DemoState>) {
    if state.last_printed == Some(state.step) {
        return;
    }
    state.last_printed = Some(state.step);
    match state.step {
        DemoStep::Intro => {
            repl_println!(
                "\x1b[36m d8b                                                               d8b \x1b[0m"
            );
            repl_println!(
                "\x1b[96m ?88                                                               88P \x1b[0m"
            );
            repl_println!(
                "\x1b[35m  88b                                                             d88  \x1b[0m"
            );
            repl_println!(
                "\x1b[33m  888888b  d8888b?88   d8P?88   d8P       88bd88b d8888b?88,.d88b,888  \x1b[0m"
            );
            repl_println!(
                "\x1b[32m  88P `?8bd8b_,dPd88  d8P'd88   88        88P'  `d8b_,dP`?88'  ?88?88  \x1b[0m"
            );
            repl_println!(
                "\x1b[34m d88,  d8888b    ?8b ,88' ?8(  d88       d88     88b      88b  d8P 88b \x1b[0m"
            );
            repl_println!(
                "\x1b[35m d88'`?88P'`?888P'`?888P'  `?88P'?8b     d88'     `?888P'  888888P'  88b\x1b[0m"
            );
            repl_println!(
                "\x1b[36m                                  )88                      88P'         \x1b[0m"
            );
            repl_println!(
                "\x1b[37m                                 ,d8P                     d88           \x1b[0m"
            );
            repl_println!(
                "\x1b[90m                              `?888P'                     ?8P           \x1b[0m"
            );

            repl_print_block(
                r#"
Welcome to the Bevy REPL demo!
This is a short interactive walkthrough of the main features of the REPL.

Quit at any time with `quit` or `Ctrl+C`.

Type `next` to proceed, or `back` to go to the previous step.
"#,
            );
        }
        DemoStep::CommandsAndAliases => {
            repl_print_block(
                r#"------
[1/6] Commands & Aliases
------
Text you type into the REPL is parsed by clap into commands and arguments.
Aliases are alternate mappings to the same underlying command.
Since we are using clap for parsing, we its whole API, including flags like --help.

Try: `say hello world` or `echo hello world`

Options:
    `--shout`           make the output uppercase
    `--repeat <number>` repeat the output <number> times
    `--help`            show help for the command

`next` to proceed.
"#,
            );
        }
        DemoStep::Logging => {
            repl_print_block(
                r#"------
[2/6] Logging
------
The REPL stays below the terminal output, including characters printed to stdout, like log messages.
In this step, try using `start` and `stop` commands to toggle a system that prints log messages to stdout.

Begin log stream: `start`
End log stream: `stop` (the log stream will end automatically after 5 seconds)

`next` to proceed.
"#,
            );
        }
        DemoStep::Events => {
            repl_print_block(
                r#"------
[3/6] Events
------
REPL commands are just events triggered by the parser.
Add an observer to handle the command event with a one-shot system.
While the REPL runs every frame in the Update schedule, the ECS lets
you react to the REPL commands in any way you want, since they are
just events.

Try: `ping`
"#,
            );
        }
        DemoStep::Ecs => {
            repl_print_block(
                r#"------
[4/6] Resources, Entities, Queries, and Bevy Commands
------
Since we program responses to the REPL commands with observers (or other event readers),
even Resources, Entities, Queries, and Bevy Commands are accessible.
Your observer has full access to the Bevy ECS and can do anything you want.

Try:
    spawn <name>        - spawn an entity with a Name component
    list                - list all entities
    query <substring>   - query for entities with a Name component containing the substring
    remove <substring>  - remove entities with a Name component containing the substring
    time                - get the current time from the Time resource

`next` to proceed. 
"#,
            );
        }
        DemoStep::Colors => {
            // Demonstrate ANSI colors rendered above the prompt
            repl_print_block(
                r#"------
[5/6] Colors
------
The REPL supports ANSI color output. This works with the crossterm backend and the pretty prompt.

Examples:"#,
            );
            repl_println!("  \x1b[31mRed\x1b[0m  \x1b[32mGreen\x1b[0m  \x1b[34mBlue\x1b[0m");
            repl_println!("  \x1b[1mBold\x1b[0m  \x1b[3mItalic\x1b[0m  \x1b[4mUnderline\x1b[0m");
            repl_println!(
                "  256-color: \x1b[38;5;208mOrange\x1b[0m  BG: \x1b[30;48;5;190m black on lime \x1b[0m"
            );
            repl_print_block(
                r#"
Tip: you can embed ANSI sequences in your own commands' output to add emphasis.

`next` to proceed.
"#,
            );
            // Also emit a few live colored lines to prove rendering
            repl_println!("\x1b[33mThis is yellow (ANSI 33)\x1b[0m");
            repl_println!("Normal line after reset.");
        }
        DemoStep::End => {
            repl_print_block(
                r#"------
[6/6] End
------
There are many more examples that demonstrate how to customize the REPL,
use clap's derive pattern to build commands, running the REPL along side
an app window, custom prompt renderers, and more.

Type `quit` to exit, or `next` to restart at the beginning.
"#,
            );
        }
    }
}

fn main() {
    // Install a global fmt layer that writes logs directly to the REPL printer,
    // preserving colors and formatting. Do this BEFORE adding DefaultPlugins.
    tracing_to_repl_fmt();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                )))
                .disable::<bevy::log::LogPlugin>(),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
            DemoPlugin,
        ))
        .run();
}
