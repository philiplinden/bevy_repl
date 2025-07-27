use bevy::prelude::*;
use crate::repl::ReplCommand;
use crate::repl::ReplResult;
use clap::{Command, Arg, ArgMatches};

/// Help command - lists all available commands
#[derive(Default)]
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

    fn execute(&self, world: &mut World, matches: &ArgMatches) -> ReplResult<String> {
        let registry = world.resource::<ReplCommandRegistry>();
        
        if let Some(cmd_name) = matches.get_one::<String>("command") {
            // Show help for specific command
            if let Some(cmd) = registry.get_command(cmd_name) {
                let help = cmd.command().render_help().to_string();
                Ok(help)
            } else {
                Ok(format!("Unknown command: {}", cmd_name))
            }
        } else {
            // List all commands
            let mut output = String::from("Available commands:\n");
            for name in registry.get_command_names() {
                if let Some(cmd) = registry.get_command(name) {
                    let command = cmd.command();
                    let about = command.get_about().unwrap_or_default();
                    output.push_str(&format!("  {:<15} {}\n", name, about.to_string()));
                }
            }
            output.push_str("\nUse 'help <command>' for detailed help on a specific command.");
            Ok(output)
        }
    }

    fn name(&self) -> &'static str {
        "help"
    }
}
