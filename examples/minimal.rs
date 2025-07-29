use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(ReplPlugin::default())
        .add_repl_command::<SpawnEnemyCommand>();

    // Run in headless mode at 60 fps
    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 60.0),
    ));

    app.run();
}

#[derive(Default)]
struct SpawnEnemyCommand;

impl ReplCommand for SpawnEnemyCommand {
    fn command(&self) -> clap::Command {
        clap::Command::new("spawn-enemy")
            .about("Spawns an enemy entity")
            .arg(clap::Arg::new("health").required(false))
    }

    fn execute(&self, commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let health = matches.get_one::<i32>("health").unwrap_or(&100);
        commands.spawn((
            Name::new("Enemy"),
            Transform::from_xyz(5.0, 0.0, 0.0),
            Health { value: *health },
        ));
        Ok(format!("Spawned enemy with health {}", health))
    }
}

// Simple component for demonstration
#[derive(Component)]
struct Health {
    value: i32,
}
