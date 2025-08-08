use bevy::app::{PluginGroup, PluginGroupBuilder};
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Use explicit module paths for clarity
            .add(bevy::log::LogPlugin::default())
            .add(bevy_ratatui::cleanup::CleanupPlugin)
            .add(bevy_ratatui::error::ErrorPlugin)
            .add(bevy_ratatui::event::EventPlugin::default())
            .add(bevy_ratatui::translation::TranslationPlugin)
            .add(crate::repl::ReplPlugin::default())
            .add(crate::prompt::PromptPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
