use crate::{
    ReplSet, input::ReplCommandQueue, registry::ReplCommandRegistry, repl_enabled,
    output::PrintReplLine,
};
use bevy::prelude::*;

pub(crate) struct ReplExecutionPlugin;

impl Plugin for ReplExecutionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ReplSet::Execution
                .run_if(repl_enabled)
                .after(ReplSet::Input),
        )
        .add_systems(Update, (command_execution,).in_set(ReplSet::Execution));
    }
}

/// System that executes commands from the registry
fn command_execution(
    mut command_queue: ResMut<ReplCommandQueue>,
    registry: Res<ReplCommandRegistry>,
    mut commands: Commands,
    mut print_events: EventWriter<PrintReplLine>,
) {
    // Process all commands in the queue
    while let Some(input) = command_queue.commands.pop_front() {
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Parse the command using clap
        if let Some(app) = registry.get_app() {
            let app_clone = app.clone();
            match app_clone.try_get_matches_from(input.split_whitespace()) {
                Ok(matches) => {
                    // Get the subcommand name
                    if let Some((command_name, sub_matches)) = matches.subcommand() {
                        if let Some(command) = registry.get_command(command_name) {
                            // Only execute commands that don't need world access
                            let result = command.execute(&mut commands, sub_matches);

                            match result {
                                Ok(output) => {
                                    if !output.is_empty() {
                                        print_events.write(PrintReplLine(output));
                                    }
                                }
                                Err(e) => {
                                    print_events.write(PrintReplLine(format!("Error: {}", e)));
                                }
                            }
                        } else {
                            print_events.write(PrintReplLine(format!("Unknown command: {}", command_name)));
                        }
                    } else {
                        // No subcommand provided, show available commands
                        let help_output = "Available commands: close, quit".to_string();
                        print_events.write(PrintReplLine(help_output));
                    }
                }
                Err(e) => {
                    print_events.write(PrintReplLine(format!("Command parse error: {}", e)));
                }
            }
        } else {
            print_events.write(PrintReplLine("No commands registered".to_string()));
        }
    }
}
