use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_ratatui::{RatatuiPlugins,
    event::EventPlugin,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    translation::TranslationPlugin,
};

/// Minimal REPL plugin group: no ratatui integration and no built-in commands.
///
/// Includes only the core REPL systems, parser, and the prompt plugin configured
/// by feature presets. This is ideal when the host app manages terminal/TUI itself
/// or wants to integrate differently.
pub struct MinimalReplPlugins;

impl PluginGroup for MinimalReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Add the core bevy_ratatui plugins to create the fallback context
            .add(EventPlugin::default())
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(TranslationPlugin)
            // Add the REPL core
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::prompt::PromptPlugin::minimal())
    }
}

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in bevy_ratatui's default
/// alternate-screen terminal context, the REPL core, prompt plugin (minimal),
/// parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Add the core bevy_ratatui plugins to create the fallback context
            .add_group(RatatuiPlugins::default())
            // Add the REPL core
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
            .add(crate::prompt::PromptPlugin::minimal())
    }
}
