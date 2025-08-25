use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in the REPL core, prompt
/// plugin, parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::prompt::PromptPlugin::default())
            .add(crate::log_ecs::ReplLogPrintPlugin)
            .add(crate::log_ecs::ReplLogRecoveryPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
