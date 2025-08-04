use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Add REPL with custom commands
    app.add_plugins(ReplPlugin)
        .repl::<SpawnEnemyCommand>(on_spawn_enemy)
        .repl::<QuitCommand>(on_quit);

    app.run();
}

/// Spawn an enemy entity with given name
#[derive(Parser)]
#[command(name = "spawn-enemy", about = "Spawn a new enemy entity")]
pub struct SpawnEnemyCommand {
    #[arg(help = "Enemy name")]
    name: String,
    
    #[arg(short, long, default_value = "100")]
    health: i32,
    
    #[arg(short, long)]
    verbose: bool,
}

impl ReplCommand for SpawnEnemyCommand {
    type Observer = fn(Trigger<Self>, Commands);
}

/// Observer function for spawning enemies
fn on_spawn_enemy(trigger: Trigger<SpawnEnemyCommand>, mut commands: Commands) {
    let cmd = trigger;
    
    if cmd.verbose {
        info!("Spawning enemy '{}' with {} health", cmd.name, cmd.health);
    }
    
    commands.spawn((
        Name::new(cmd.name.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Enemy { health: cmd.health },
    ));
    
    info!("Spawned enemy '{}' with {} health", cmd.name, cmd.health);
}

/// Observer function for quitting
fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    if trigger.verbose {
        info!("Quitting...");
    }
    exit.send(AppExit::Success);
}

/// Enemy component
#[derive(Component)]
struct Enemy {
    health: i32,
} 