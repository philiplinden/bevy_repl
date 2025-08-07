use bevy::prelude::*;
use bevy_ratatui::{
    context::TerminalContext,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    event::EventPlugin,
    translation::TranslationPlugin,
};
use std::io::{Stdout, stdout};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use bevy::app::{Plugin, PluginGroup, PluginGroupBuilder, Startup};

use crate::{parse::ParserPlugin, prompt::PromptPlugin, repl::ReplPlugin};


pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(EventPlugin::default())
            .add(TranslationPlugin)
            .add(ReplPlugin::default())
            .add(ReplContextPlugin)
            .add(PromptPlugin::default())
            .add(ParserPlugin)
    }
}

/// `bevy_ratatui` always creates a new terminal screen when setting up the
/// context. This plugin is used to create a single terminal screen that is
/// shared between the REPL and the main application so the logs and other
/// normal outputs are visible in the same terminal.
pub struct ReplContextPlugin;

impl Plugin for ReplContextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, context_setup);
    }
}

/// A startup system that sets up the terminal context.
pub fn context_setup(mut commands: Commands) -> Result {
    let terminal = ReplContext::init()?;
    commands.insert_resource(terminal);

    Ok(())
}

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct ReplContext(Terminal<CrosstermBackend<Stdout>>);

impl TerminalContext<CrosstermBackend<Stdout>> for ReplContext {
    fn init() -> Result<Self> {
        let stdout = stdout();
        // Enable raw mode but stay in main screen
        bevy_ratatui::crossterm::terminal::enable_raw_mode().unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        Ok(Self(terminal))
    }

    fn restore() -> Result<()> {
        bevy_ratatui::crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
    fn configure_plugin_group(
        _group: &bevy_ratatui::RatatuiPlugins,
        builder: bevy::app::PluginGroupBuilder,
    ) -> bevy::app::PluginGroupBuilder {
        builder
    }
}
