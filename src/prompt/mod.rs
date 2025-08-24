pub mod input;
pub mod renderer;
pub mod key_events;
pub mod scroll;

use bevy::prelude::*;
use std::sync::Arc;

use super::repl::{ReplSet};
use self::input::PromptInputPlugin;
use self::key_events::block_keyboard_input_forwarding;
use self::renderer::{PromptRenderer, PromptRenderPlugin};
use self::scroll::ScrollRegionPlugin;

/// Visual configuration for the REPL prompt bar.
#[derive(Resource, Clone)]
pub struct PromptPlugin {
    pub config: ReplPromptConfig,
    pub renderer: Arc<dyn PromptRenderer>,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self {
            config: ReplPromptConfig::default(),
            renderer: Arc::new(renderer::simple::SimpleRenderer),
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplPrompt {
            symbol: Some(self.config.symbol.clone().unwrap_or_else(|| "> ".to_string())),
            buffer: String::new(),
        });
        app.insert_resource(self.config.clone());
        app.add_plugins((
            PromptInputPlugin,
            PromptRenderPlugin { renderer: self.renderer.clone() },
            ScrollRegionPlugin,
        ));
        app.add_systems(
            Update,
            (
                block_keyboard_input_forwarding
                    .in_set(ReplSet::Post)
                    .in_set(ReplSet::All)
                    .after(ReplSet::Render),
            ),
        );
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub buffer: String,
}

#[derive(Resource, Clone)]
pub struct ReplPromptConfig {
    pub symbol: Option<String>,
}

impl Default for ReplPromptConfig {
    fn default() -> Self {
        Self {
            symbol: Some("> ".to_string()),
        }
    }
}
