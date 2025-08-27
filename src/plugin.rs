use bevy::{app::{PluginGroup, PluginGroupBuilder}};
use bevy_ratatui::{
    event::EventPlugin,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    translation::TranslationPlugin,
};

/// Minimal Ratatui plugin group that replicates the default Ratatui plugin
/// group but without the alternate screen because for some reason all of the
/// built-in Ratatui plugin groups want to create an alternate screen.
///
/// Use this plugin group OR [`bevy_ratatui::RatatuiPlugins`] but NOT BOTH. If you
/// want to use the alternate screen, use [`bevy_ratatui::RatatuiPlugins`] and
/// disable [`StdoutRatatuiPlugins`] in the [`ReplPlugins`] group.
///
/// ```rust
/// app.add_plugins(
///     bevy_repl::ReplPlugins.build()
///         .disable::<bevy_repl::StdoutRatatuiPlugins>(),
///     bevy_ratatui::RatatuiPlugins::default(),
/// );
/// ```
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
/// This is the turnkey setup most users want. It wires in the REPL core, prompt
/// plugin, parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(StdoutRatatuiPlugins)
            .add(crate::context::ReplContextPlugin)
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::prompt::PromptPlugin::default())
            .add(crate::log_ecs::ReplLogPrintPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
