use bevy::prelude::*;
use crate::repl::ReplCommand;
use crate::repl::ReplResult;
use clap::{Command, ArgMatches};

/// System info command - show system information
#[derive(Default)]
pub struct SysInfoCommand;

impl ReplCommand for SysInfoCommand {
    fn command(&self) -> Command {
        Command::new("sysinfo")
            .about("Show system information")
    }

    fn execute(&self, world: &mut World, _matches: &ArgMatches) -> ReplResult<String> {
        let mut output = String::new();
        
        // Basic system information
        output.push_str("System Information:\n");
        output.push_str("==================\n\n");
        
        // Entity count
        let entity_count = world.iter_entities().count();
        output.push_str(&format!("Total Entities: {}\n", entity_count));
        
        // Component count
        let component_count = world.components().len();
        output.push_str(&format!("Total Components: {}\n", component_count));
        
        // Resource count
        let resource_count = world.resources().len();
        output.push_str(&format!("Total Resources: {}\n", resource_count));
        
        // Archetype count
        let archetype_count = world.archetypes().len();
        output.push_str(&format!("Total Archetypes: {}\n", archetype_count));
        
        // Memory usage (approximate)
        let total_memory = world.archetypes()
            .map(|archetype| archetype.len() * archetype.layout().size())
            .sum::<usize>();
        output.push_str(&format!("Approximate Memory Usage: {} bytes\n", total_memory));
        
        Ok(output)
    }

    fn name(&self) -> &'static str {
        "sysinfo"
    }
}
