use bevy::prelude::*;
use bevy_repl::prelude::*;

// This example shows how to use the derive feature
// To enable this, add the "derive" feature to your Cargo.toml:
// bevy_repl = { version = "0.1.0", features = ["derive"] }

fn main() {
    App::new()
        .add_plugins(ReplPlugin::default())
        // Register custom commands using the derive approach
        .add_repl_command::<SpawnPlayerCommand>()
        .add_repl_command::<TeleportCommand>()
        .add_repl_command::<ListEntitiesCommand>()
        .run();
}

// ============================================================================
// DERIVE APPROACH (Derive Feature) - Automatic Implementation
// ============================================================================

/// Spawn a player entity with given name
#[derive(ReplCommand, Default)]
#[command(name = "spawn-player", about = "Spawn a new player entity")]
pub struct SpawnPlayerCommand {
    #[arg(help = "Player name", required = true)]
    name: String,
    
    #[arg(help = "Starting health", default_value = "100")]
    health: i32,
}

impl SpawnPlayerCommand {
    fn execute(&self, commands: &mut Commands) -> ReplResult<String> {
        commands.spawn((
            Name::new(self.name.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player { health: self.health },
        ));

        Ok(format!("Spawned player '{}' with {} health", self.name, self.health))
    }
}

/// Teleport an entity to specific coordinates
#[derive(ReplCommand, Default)]
#[command(name = "teleport", about = "Teleport an entity to coordinates")]
pub struct TeleportCommand {
    #[arg(help = "Entity ID to teleport", required = true)]
    entity: u32,
    
    #[arg(help = "X coordinate", required = true)]
    x: f32,
    
    #[arg(help = "Y coordinate", required = true)]
    y: f32,
    
    #[arg(help = "Z coordinate", default_value = "0")]
    z: f32,
}

impl TeleportCommand {
    fn execute(&self, _commands: &mut Commands) -> ReplResult<String> {
        // In a real implementation, you'd query for the entity and update its transform
        Ok(format!("Teleported entity {} to ({}, {}, {})", self.entity, self.x, self.y, self.z))
    }
}

/// List all entities in the world
#[derive(ReplCommand, Default)]
#[command(name = "list-entities", about = "List all entities in the world")]
pub struct ListEntitiesCommand {
    #[arg(help = "Filter by component type", long, short)]
    filter: Option<String>,
}

impl ListEntitiesCommand {
    fn execute(&self, _commands: &mut Commands) -> ReplResult<String> {
        if let Some(filter_type) = &self.filter {
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

// ============================================================================
// ADVANCED DERIVE FEATURES (Hypothetical)
// ============================================================================

/// Example of more advanced derive features that could be implemented
#[derive(ReplCommand, Default)]
#[command(name = "advanced", about = "Advanced command with validation")]
pub struct AdvancedCommand {
    #[arg(help = "Player name", required = true)]
    #[validate(length(min = 3, max = 20))]
    name: String,
    
    #[arg(help = "Health value", required = true)]
    #[validate(range(min = 1, max = 1000))]
    health: i32,
    
    #[arg(help = "Position", required = true)]
    #[validate(custom = "validate_position")]
    position: Vec3,
    
    #[arg(help = "Enable debug mode", long, short)]
    debug: bool,
}

impl AdvancedCommand {
    fn execute(&self, commands: &mut Commands) -> ReplResult<String> {
        if self.debug {
            println!("Debug: Spawning player with position {:?}", self.position);
        }
        
        commands.spawn((
            Name::new(self.name.clone()),
            Transform::from_translation(self.position),
            Player { health: self.health },
        ));

        Ok(format!("Spawned advanced player '{}'", self.name))
    }
}

// Custom validation function (would be part of the derive macro)
fn validate_position(pos: &Vec3) -> Result<(), String> {
    if pos.x.abs() > 1000.0 || pos.y.abs() > 1000.0 || pos.z.abs() > 1000.0 {
        Err("Position coordinates must be within ±1000".to_string())
    } else {
        Ok(())
    }
} 
