//! Example showing that observer functions can run ECS Queries.
//! 
//! Demonstrates:
//! - Using a REPL command derived with clap
//! - Accessing a `Query` inside the observer function
//! - Listing entities and optionally filtering by `Name`

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

/// List entities, optionally filtering by a substring of their Name component.
#[derive(Debug, Clone, Event, Default)]
struct ListCommand {
    /// Optional substring to filter `Name` by
    name_contains: Option<String>,
}

impl ReplCommand for ListCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("list")
            .about("List entities, optionally filter by name substring")
            .arg(
                clap::Arg::new("name_contains")
                    .short('n')
                    .long("name-contains")
                    .num_args(1)
                    .required(false)
                    .help("Optional substring to filter Name by"),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> bevy_repl::command::ReplResult<Self> {
        let name_contains = matches.get_one::<String>("name_contains").cloned();
        Ok(ListCommand { name_contains })
    }
}

/// Observer demonstrating a read-only ECS query inside the handler.
fn on_list(trigger: Trigger<ListCommand>, query: Query<(Entity, Option<&Name>)>) {
    let cmd = trigger.event();
    let needle = cmd.name_contains.as_deref();

    repl_println!("Entities:");
    let mut count = 0usize;
    for (entity, name_opt) in query.iter() {
        let name_str = name_opt.map(|n| n.as_str()).unwrap_or("<unnamed>");
        if let Some(substr) = needle {
            if !name_str.contains(substr) {
                continue;
            }
        }
        repl_println!("  {:?}: {}", entity, name_str);
        count += 1;
    }
    repl_println!("Total listed: {}", count);
}

/// Spawn some example entities so we have something to list.
fn spawn_entities(mut commands: Commands) {
    commands.spawn(Name::new("Alice"));
    commands.spawn(Name::new("Bob"));
    commands.spawn(Name::new("Carol"));
    commands.spawn_empty(); // unnamed
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL query example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `list`                         - List all entities");
    repl_println!("  `list -n Al`                   - List entities whose name contains 'Al'");
    repl_println!("  `quit`                         - Close the app");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            bevy::input::InputPlugin::default(),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
        ))
        .add_repl_command::<ListCommand>()
        .add_observer(on_list)
        .add_systems(Startup, spawn_entities)
        .add_systems(PostStartup, instructions)
        .run();
}
