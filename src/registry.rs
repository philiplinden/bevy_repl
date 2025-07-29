use bevy::prelude::*;
use std::collections::HashMap;
use crate::{ReplCommand, ReplResult};

#[derive(Resource, Default)]
pub struct ReplCommandRegistry {
    commands: HashMap<String, Box<dyn ReplCommand>>,
    app: Option<clap::Command>, // Built dynamically from registered commands
}

impl ReplCommandRegistry {
    pub fn get_command_names(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }
    
    pub fn get_app(&self) -> Option<&clap::Command> {
        self.app.as_ref()
    }
    
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }
    
    pub fn register(&mut self, command: impl ReplCommand + Clone + 'static) {
        let name = command.command().get_name().to_string();
        let command_clone = command.clone();
        self.commands.insert(name.clone(), Box::new(command_clone));
        self.rebuild_app();
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn ReplCommand>> {
        self.commands.get(name)
    }

    fn rebuild_app(&mut self) {
        let mut app = clap::Command::new("bevy_repl")
            .about("Bevy REPL Commands")
            .subcommand_negates_reqs(true)
            .arg_required_else_help(false);

        for (_, command) in &self.commands {
            app = app.subcommand(command.command());
        }

        self.app = Some(app);
    }

    pub fn parse_and_execute(&self, input: &str, commands: &mut Commands) -> ReplResult<String> {
        if let Some(app) = &self.app {
            let matches = app.clone().try_get_matches_from(input.split_whitespace())?;
            
            if let Some((name, sub_matches)) = matches.subcommand() {
                if let Some(command) = self.get_command(name) {
                    command.as_ref().execute(commands, sub_matches)
                } else {
                    Ok(format!("Unknown command: `{}`. Use 'help' to see available commands.", name))
                }
            } else {
                // No command specified
                Ok("".to_string())
            }
        } else {
            Ok("No commands registered. Register commands with `add_repl_command`.".to_string())
        }
    }


}

/// Add clap commands to the Bevy app.
pub trait ReplCommandRegistration {
    /// Add a REPL command.
    ///
    /// This registers the console command so it will print with the built-in `help` command.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_repl::{ReplCommandRegistration, ReplCommand};
    /// App::new()
    ///     .add_repl_command::<LogCommand>();
    /// #
    /// # /// Prints given arguments to the console.
    /// # #[derive(Default)]
    /// # struct LogCommand;
    /// # impl ReplCommand for LogCommand {
    /// #     fn command(&self) -> clap::Command { clap::Command::new("log") }
    /// #     fn execute(&self, _: &mut Commands, _: &clap::ArgMatches) -> ReplResult<String> { Ok("".to_string()) }
    /// # }
    /// ```
    fn add_repl_command<T: ReplCommand + Default + Clone + 'static>(&mut self) -> &mut Self;
}

impl ReplCommandRegistration for App {
    fn add_repl_command<T: ReplCommand + Default + Clone + 'static>(&mut self) -> &mut Self {
        self.add_systems(Startup, |mut registry: ResMut<ReplCommandRegistry>| {
            registry.register(T::default());
        });
        self
    }
}
