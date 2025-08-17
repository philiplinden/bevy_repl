pub mod input;
pub mod key_events;
pub mod renderer;
pub mod config;

use bevy::prelude::*;
use std::sync::Arc;

use self::input::PromptInputPlugin;
use self::key_events::block_keyboard_input_forwarding;
use self::renderer::{PromptRenderPlugin, PromptRenderer};
use self::config::ReplPromptConfig;
use crate::repl::ReplSet;

#[derive(Clone)]
pub enum PromptMode {
    InFrame,
    #[cfg(feature = "pretty")]
    AlternateScreen,
}

#[derive(Resource, Clone)]
pub struct PromptPlugin {
    pub config: ReplPromptConfig,
    pub renderer: Arc<dyn PromptRenderer>,
    pub mode: PromptMode,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        #[cfg(feature = "pretty")]
        return Self::simple();

        #[cfg(not(feature = "pretty"))]
        return Self::minimal();
    }
}

impl PromptPlugin {
    pub fn minimal() -> Self {
        Self {
            config: ReplPromptConfig::minimal(),
            renderer: Arc::new(renderer::minimal::MinimalRenderer),
            mode: PromptMode::InFrame,
        }
    }

    #[cfg(feature = "pretty")]
    pub fn simple() -> Self {
        Self {
            config: ReplPromptConfig::simple(),
            renderer: Arc::new(renderer::pretty::PrettyRenderer),
            mode: PromptMode::AlternateScreen,
        }
    }

    #[cfg(feature = "pretty")]
    pub fn pretty() -> Self {
        Self {
            config: ReplPromptConfig::pretty(),
            renderer: Arc::new(renderer::pretty::PrettyRenderer),
            mode: PromptMode::AlternateScreen,
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplPrompt {
            symbol: self.config.symbol.clone(),
            buffer: String::new(),
        });
        app.insert_resource(self.config.clone());
        app.add_plugins(PromptInputPlugin);
        app.add_plugins(PromptRenderPlugin {
            renderer: self.renderer.clone(),
        });
        // Logging behavior depends on mode:
        // - In minimal mode, print logs to stdout via a system (no in-frame log buffer)
        // - In pretty/alternate-screen mode, use in-frame log buffer for rendering
        match self.mode {
            PromptMode::InFrame => {
                // Capture tracing -> ECS events and print them above the prompt via stdout
                app.add_plugins(crate::log_ecs::CaptureSubscriberPlugin::default());
                app.add_systems(Update, crate::log_ecs::print_log_events_system);
            }
            #[cfg(feature = "pretty")]
            PromptMode::AlternateScreen => {
                // Provide a buffer of recent logs for the renderer to draw
                // in-frame on a dedicated ratatui screen
                app.add_plugins(crate::log_ecs::LogBufferPlugin::default());
            }
        }
        app.add_systems(
            Update,
            (block_keyboard_input_forwarding
                .in_set(ReplSet::Post)
                .in_set(ReplSet::All)
                .after(ReplSet::Render),),
        );
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub buffer: String,
}
