use bevy::app::{PluginGroup, PluginGroupBuilder};

use bevy_ratatui::{
    event::EventPlugin,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    translation::TranslationPlugin,
};

/// Minimal Ratatui plugin group: replicates the default Ratatui plugin group
/// but without the alternate screen.
pub struct StdoutRatatuiPlugins;

impl PluginGroup for StdoutRatatuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EventPlugin::default())
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(TranslationPlugin)
    }
}

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in bevy_ratatui's default
/// alternate-screen terminal context, the REPL core, prompt plugin, parser,
/// and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
            .add(crate::prompt::PromptPlugin::default())
    }
}
