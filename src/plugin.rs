use bevy::app::{PluginGroup, PluginGroupBuilder};
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        // Configure the prompt plugin via presets
        #[cfg(feature = "pretty")]
        let prompt_plugin = crate::prompt::PromptPlugin { config: crate::prompt::ReplPromptConfig::pretty() };
        #[cfg(not(feature = "pretty"))]
        let prompt_plugin = crate::prompt::PromptPlugin { config: crate::prompt::ReplPromptConfig::minimal() };

        let builder = PluginGroupBuilder::start::<Self>()
            // Use explicit module paths for clarity
            .add(bevy_ratatui::cleanup::CleanupPlugin)
            .add(bevy_ratatui::error::ErrorPlugin)
            .add(bevy_ratatui::event::EventPlugin::default())
            .add(bevy_ratatui::translation::TranslationPlugin)
            .add(crate::repl::ReplPlugin::default())
            .add(prompt_plugin)
            .add(crate::command::ParserPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin);

        builder
    }
}
