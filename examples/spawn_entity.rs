//! Command example that spawns an entity via the REPL.
//! 
//! Demonstrates:
//! - Defining a command with clap's derive macros
//! - Automatic `ReplCommand` via `#[derive(ReplCommand)]`
//! - Spawning an entity from an observer using `Commands`

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

/// Spawn an entity with an optional `Name` component.
#[derive(Debug, Clone, Event, Default)]
struct SpawnEntityCommand {
    /// Optional name to attach to the spawned entity
    name: Option<String>,
}

impl ReplCommand for SpawnEntityCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("spawn")
            .about("Spawn an entity, optionally with a name")
            .arg(
                clap::Arg::new("name")
                    .short('n')
                    .long("name")
                    .num_args(1)
                    .required(false)
                    .help("Optional name to attach to the spawned entity"),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> bevy_repl::command::ReplResult<Self> {
        let name = matches.get_one::<String>("name").cloned();
        Ok(SpawnEntityCommand { name })
    }
}

/// Observer that handles the `spawn` command and spawns into the ECS.
fn on_spawn(trigger: Trigger<SpawnEntityCommand>, mut commands: Commands) {
    let cmd = trigger.event();

    // Build the entity
    let mut e = commands.spawn_empty();
    if let Some(n) = &cmd.name {
        e.insert(Name::new(n.clone()));
    }

    let id = e.id();
    println!("Spawned entity with id: {:?}", id);
}

fn instructions() {
    println!();
    println!("Welcome to the Bevy REPL spawn example!");
    println!();
    println!("Try typing a command:");
    println!("  `spawn`                  - Spawn an unnamed entity");
    println!("  `spawn -n Alice`         - Spawn an entity named 'Alice'");
    println!("  `quit`                   - Close the app");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            ReplPlugins,
        ))
        .add_repl_command::<SpawnEntityCommand>()
        .add_observer(on_spawn)
        .add_systems(Startup, instructions)
        .run();
}
