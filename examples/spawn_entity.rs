//! Command example that spawns an entity via the REPL.
//! 
//! Demonstrates:
//! - Defining a command with clap's derive macros
//! - Automatic `ReplCommand` via `#[derive(ReplCommand)]`
//! - Spawning an entity from an observer using `Commands`

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use clap::Parser;

/// Spawn an entity with an optional `Name` component.
#[derive(Parser, ReplCommand, Debug, Clone, Event, Default)]
#[command(name = "spawn", about = "Spawn an entity, optionally with a name")] 
struct SpawnEntityCommand {
    /// Optional name to attach to the spawned entity
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
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
