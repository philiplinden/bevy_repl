use bevy::prelude::*;
use crate::{ReplCommand, ReplResult, ReplCommandRegistry};
use clap::{Command, Arg, ArgMatches};

/// Help command - lists all available commands
#[derive(Default, Clone)]
pub struct HelpCommand;

impl ReplCommand for HelpCommand {
    fn command(&self) -> Command {
        Command::new("help")
            .about("List all available commands")
            .arg(
                Arg::new("command")
                    .help("Show help for a specific command")
                    .required(false)
                    .num_args(0..=1)
            )
    }

    fn execute_with_world(&self, world: &World, _commands: &mut Commands, matches: &ArgMatches) -> ReplResult<String> {
        if let Some(cmd_name) = matches.get_one::<String>("command") {
            if let Some(cmd) = world.resource::<ReplCommandRegistry>().get_command(cmd_name) {
                let help = cmd.command().render_help().to_string();
                Ok(help)
            } else {
                Ok(format!("Unknown command: {}", cmd_name))
            }
        } else {
            let mut output = String::from("Available commands:\n");
            for name in world.resource::<ReplCommandRegistry>().get_command_names() {
                if let Some(cmd) = world.resource::<ReplCommandRegistry>().get_command(name) {
                    let command = cmd.command();
                    let about = command.get_about().unwrap_or_default();
                    output.push_str(&format!("  {:<15} {}\n", name, about.to_string()));
                }
            }
            output.push_str("\nUse 'help <command>' for detailed help on a specific command.");
            Ok(output)
        }
    }
}
