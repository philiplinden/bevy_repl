use bevy::prelude::*;
use bevy_repl::prelude::*;

// This example shows the basic derive feature usage
// For more advanced clap-style syntax, see clap_style_derive.rs
// To enable this, add the "derive" feature to your Cargo.toml:
// bevy_repl = { version = "0.1.0", features = ["derive"] }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::default())
        // Register custom commands using the derive approach
        .add_repl_command::<HelloCommand>()
        .add_repl_command::<StatusCommand>()
        .add_repl_command::<CustomCommand>()
        .run();
}

/// Simple hello command
#[derive(ReplCommand)]
#[command(name = "hello", about = "Say hello to the world")]
pub struct HelloCommand;

/// Status command with custom implementation
#[derive(ReplCommand)]
#[command(name = "status", about = "Show application status")]
pub struct StatusCommand;

impl StatusCommand {
    // You can still override the default execute method
    fn execute(&self, _commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        Ok("Application is running smoothly!".to_string())
    }
}

/// Custom command with aliases
#[derive(ReplCommand)]
#[command(name = "custom", about = "A custom command with aliases", aliases = ["c", "cust"])]
pub struct CustomCommand;

impl CustomCommand {
    fn execute(&self, _commands: &mut Commands, _matches: &clap::ArgMatches) -> ReplResult<String> {
        Ok("Custom command executed! Available aliases: c, cust".to_string())
    }
}
