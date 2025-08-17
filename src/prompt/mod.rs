pub mod config;
pub mod input;
pub mod key_events;
pub mod renderer;

use bevy::prelude::*;
use std::sync::Arc;

use self::config::ReplPromptConfig;
use self::input::PromptInputPlugin;
use self::key_events::block_keyboard_input_forwarding;
use self::renderer::{PromptRenderPlugin, PromptRenderer};

use crate::repl::ReplSet;

#[derive(Resource, Clone)]
pub struct PromptPlugin {
    pub config: ReplPromptConfig,
    pub renderer: Arc<dyn PromptRenderer>,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self::simple()
    }
}

impl PromptPlugin {
    pub fn stdout() -> Self {
        Self {
            config: ReplPromptConfig::minimal(),
            renderer: Arc::new(renderer::stdout::StdoutRenderer),
        }
    }

    pub fn simple() -> Self {
        Self {
            config: ReplPromptConfig::simple(),
            renderer: Arc::new(renderer::alt_screen::AltScreenRenderer),
        }
    }

    pub fn pretty() -> Self {
        Self {
            config: ReplPromptConfig::pretty(),
            renderer: Arc::new(renderer::alt_screen::AltScreenRenderer),
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PromptInputPlugin);
        app.add_systems(Update, block_keyboard_input_forwarding.in_set(ReplSet::All));

        // Let the selected renderer install its logging pipeline
        let renderer = self.renderer.clone();
        renderer.configure_logging(app);
        app.add_plugins(PromptRenderPlugin { renderer });

        app.insert_resource(self.config.clone());
    }
}
