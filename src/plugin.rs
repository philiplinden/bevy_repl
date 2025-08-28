use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ratatui::{
    cleanup::CleanupPlugin, error::ErrorPlugin, event::EventPlugin, translation::TranslationPlugin,
};

/// Minimal Ratatui plugin group that replicates the default Ratatui plugin
/// group but without the alternate screen because for some reason all of the
/// built-in Ratatui plugin groups want to create an alternate screen.
///
/// This plugin is configured to be added to the [`ReplPlugins`] group by
/// default. If the base Ratatui Plugins are already added to the app, this
/// plugin will not add them.
pub struct StdoutRatatuiPlugin;

impl Plugin for StdoutRatatuiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EventPlugin>() {
            app.add_plugins(EventPlugin::default());
        }
        if !app.is_plugin_added::<CleanupPlugin>() {
            app.add_plugins(CleanupPlugin);
        }
        if !app.is_plugin_added::<ErrorPlugin>() {
            app.add_plugins(ErrorPlugin);
        }
        if !app.is_plugin_added::<TranslationPlugin>() {
            app.add_plugins(TranslationPlugin);
        }
    }
}

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in the REPL core, prompt
/// plugin, parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StdoutRatatuiPlugin)
            .add(crate::context::ReplContextPlugin)
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::prompt::PromptPlugin::default())
            .add(crate::log_ecs::ReplLogPrintPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
