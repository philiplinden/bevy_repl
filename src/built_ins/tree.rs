
/// Tree command - list entities with components as a tree
#[derive(Default)]
pub struct TreeCommand;

impl ReplCommand for TreeCommand {
    fn name(&self) -> &'static str {
        "tree"
    }

    fn command(&self) -> Command {
        Command::new("tree")
            .about("List entities with components as a tree")
            .arg(
                Arg::new("entity")
                    .help("Show tree for specific entity ID")
                    .required(false)
                    .num_args(0..=1)
            )
    }

    // TreeCommand entity iteration fix
    fn execute(&self, world: &mut World, _matches: &clap::ArgMatches) -> ReplResult<String> {
        let mut output = String::from("Entity Tree:\n");
        
        // Correct Bevy 0.12+ approach
        let mut query = world.query::<Entity>();
        for entity in query.iter(world) {
            output.push_str(&format!("  Entity {:?}\n", entity));
            
            // Get archetype to list components
            if let Ok(entity_ref) = world.get_entity(entity) {
                let archetype = entity_ref.archetype();
                for component in archetype.components() {
                    if let Some(info) = world.components().get_info(component) {
                        output.push_str(&format!("    - {}\n", info.name()));
                    }
                }
            }
        }
        Ok(output)
    }
}