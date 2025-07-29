use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    App::new()
        .add_plugins(ReplPlugin::default())
        // Register custom commands using the basic approach
        .add_repl_command::<SpawnPlayerCommand>()
        .add_repl_command::<TeleportCommand>()
        .add_repl_command::<ListEntitiesCommand>()
        .run();
}

// ============================================================================
// BASIC APPROACH (Default Feature) - Manual Implementation
// ============================================================================

/// Spawn a player entity with given name
#[derive(Default, Clone)]
pub struct SpawnPlayerCommand;

impl ReplCommand for SpawnPlayerCommand {
    fn command(&self) -> clap::Command {
        clap::Command::new("spawn-player")
            .about("Spawn a new player entity")
            .arg(
                clap::Arg::new("name")
                    .help("Player name")
                    .required(true)
                    .value_name("NAME")
            )
            .arg(
                clap::Arg::new("health")
                    .help("Starting health")
                    .required(false)
                    .value_name("HEALTH")
                    .default_value("100")
            )
    }

    fn execute(&self, commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let name = matches.get_one::<String>("name").unwrap();
        let health: i32 = matches.get_one::<String>("health")
            .unwrap()
            .parse()
            .map_err(|e| format!("Invalid health value: {}", e))?;

        commands.spawn((
            Name::new(name.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player { health },
        ));

        Ok(format!("Spawned player '{}' with {} health", name, health))
    }
}

/// Teleport an entity to specific coordinates
#[derive(Default, Clone)]
pub struct TeleportCommand;

impl ReplCommand for TeleportCommand {
    fn command(&self) -> clap::Command {
        clap::Command::new("teleport")
            .about("Teleport an entity to coordinates")
            .arg(
                clap::Arg::new("entity")
                    .help("Entity ID to teleport")
                    .required(true)
                    .value_name("ENTITY_ID")
            )
            .arg(
                clap::Arg::new("x")
                    .help("X coordinate")
                    .required(true)
                    .value_name("X")
            )
            .arg(
                clap::Arg::new("y")
                    .help("Y coordinate")
                    .required(true)
                    .value_name("Y")
            )
            .arg(
                clap::Arg::new("z")
                    .help("Z coordinate")
                    .required(false)
                    .value_name("Z")
                    .default_value("0")
            )
    }

    fn execute(&self, _commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let entity_id: u32 = matches.get_one::<String>("entity")
            .unwrap()
            .parse()
            .map_err(|e| format!("Invalid entity ID: {}", e))?;
        
        let x: f32 = matches.get_one::<String>("x")
            .unwrap()
            .parse()
            .map_err(|e| format!("Invalid X coordinate: {}", e))?;
        
        let y: f32 = matches.get_one::<String>("y")
            .unwrap()
            .parse()
            .map_err(|e| format!("Invalid Y coordinate: {}", e))?;
        
        let z: f32 = matches.get_one::<String>("z")
            .unwrap()
            .parse()
            .map_err(|e| format!("Invalid Z coordinate: {}", e))?;

        // In a real implementation, you'd query for the entity and update its transform
        // For this example, we'll just return a success message
        Ok(format!("Teleported entity {} to ({}, {}, {})", entity_id, x, y, z))
    }
}

/// List all entities in the world
#[derive(Default, Clone)]
pub struct ListEntitiesCommand;

impl ReplCommand for ListEntitiesCommand {
    fn command(&self) -> clap::Command {
        clap::Command::new("list-entities")
            .about("List all entities in the world")
            .arg(
                clap::Arg::new("filter")
                    .help("Filter by component type")
                    .required(false)
                    .value_name("COMPONENT")
                    .long("filter")
                    .short('f')
            )
    }

    fn execute(&self, _commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let filter = matches.get_one::<String>("filter");
        
        if let Some(filter_type) = filter {
            Ok(format!("Listing entities with component: {}", filter_type))
        } else {
            Ok("Listing all entities in the world".to_string())
        }
    }
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
struct Player {
    health: i32,
} 
