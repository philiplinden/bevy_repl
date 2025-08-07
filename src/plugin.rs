use bevy_ratatui::{
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    event::EventPlugin,
    translation::TranslationPlugin,
};

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::*;

pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(EventPlugin::default())
            .add(TranslationPlugin)
            .add(ReplPlugin::default())
            .add(PromptPlugin::default())
            .add(ParserPlugin)
    }
}

