use bevy::{app::{PluginGroup, PluginGroupBuilder}, prelude::*};

/// Minimal REPL plugin group: no ratatui integration and no built-in commands.
///
/// Includes only the core REPL systems, parser, and the prompt plugin configured
/// by feature presets. This is ideal when the host app manages terminal/TUI itself
/// or wants to integrate differently.
pub struct MinimalReplPlugins;

impl PluginGroup for MinimalReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        // Configure the prompt plugin via presets
        #[cfg(feature = "pretty")]
        let prompt_plugin = crate::prompt::PromptPlugin { config: crate::prompt::ReplPromptConfig::pretty() };
        #[cfg(not(feature = "pretty") )]
        let prompt_plugin = crate::prompt::PromptPlugin { config: crate::prompt::ReplPromptConfig::minimal() };

        PluginGroupBuilder::start::<Self>()
            // Add minimal bevy_ratatui plugins needed for KeyEvent handling
            .add(bevy_ratatui::event::EventPlugin::default())
            .add(bevy_ratatui::cleanup::CleanupPlugin)
            // Add Bevy's input plugin for keyboard input blocking system
            .add(bevy::input::InputPlugin)
            .add(crate::repl::ReplPlugin::default())
            .add(prompt_plugin)
            .add(crate::command::ParserPlugin)
    }
}

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in bevy_ratatui's default
/// alternate-screen terminal context, the REPL core, prompt plugin (pretty/minimal
/// by feature), parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(bevy_ratatui::RatatuiPlugins::default())
            .add_group(MinimalReplPlugins)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
