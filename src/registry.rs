use bevy::{
    prelude::*,
    ecs::{
    system::{ScheduleSystem, SystemMeta, SystemParam},
}};
use std::collections::HashMap;
use crate::{
    repl::{ReplCommand, ReplResult, ReplSet},
    config::ReplConfig,
};



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
    
    pub fn register(&mut self, command: impl ReplCommand + 'static) {
        let name = command.name().to_string();
        self.commands.insert(name, Box::new(command));
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

    pub fn parse_and_execute(&self, input: &str, world: &mut World) -> ReplResult<String> {
        if let Some(app) = &self.app {
            let matches = app.clone().try_get_matches_from(input.split_whitespace())?;
            
            if let Some((name, sub_matches)) = matches.subcommand() {
                if let Some(command) = self.get_command(name) {
                    command.execute(world, sub_matches)
                } else {
                    Ok(format!("Unknown command: {}", name))
                }
            } else {
                Ok("No command specified. Use 'help' to see available commands.".to_string())
            }
        } else {
            Ok("No commands registered. Register commands with `register_command`.".to_string())
        }
    }
}

/// Add clap commands to the Bevy app.
pub trait ReplCommandRegistration {
    /// Add a REPL command with a given system.
    ///
    /// This registers the console command so it will print with the built-in `help` command.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_repl::{AddReplCommand, ReplCommand};
    /// # use clap::Parser;
    /// App::new()
    ///     .add_repl_command::<LogCommand, _>(log_command);
    /// #
    /// # /// Prints given arguments to the console.
    /// # #[derive(Parser, ReplCommand)]
    /// # #[command(name = "log")]
    /// # struct LogCommand;
    /// #
    /// # fn log_command(mut log: ReplCommand<LogCommand>) {}
    /// ```
    fn add_repl_command<T: Command, Params>(
        &mut self,
        system: impl IntoScheduleConfigs<ScheduleSystem, Params>,
    ) -> &mut Self;
}

impl ReplCommandRegistration for App {
    fn add_repl_command<T: Command, Params>(
        &mut self,
        system: impl IntoScheduleConfigs<ScheduleSystem, Params>,
    ) -> &mut Self {
        let sys = move |mut config: ResMut<ReplConfig>| {
            let command = T::command().no_binary_name(true);
            // .color(clap::ColorChoice::Always);
            let name = T::name();
            if config.commands.contains_key(name) {
                warn!(
                    "console command '{}' already registered and was overwritten",
                    name
                );
            }
            config.commands.insert(name, command);
        };

        self.add_systems(Startup, sys.in_set(ReplSet::Startup))
            .add_systems(Update, system.in_set(ReplSet::Commands))
    }
}
