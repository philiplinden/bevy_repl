pub mod input;
pub mod renderer;
pub mod key_events;

use bevy::prelude::*;
use std::sync::Arc;

use crate::repl::ReplSet;
use crate::log_ecs::InFrameLogPlugin;
use self::input::PromptInputPlugin;
use self::key_events::block_keyboard_input_forwarding;
use self::renderer::{PromptRenderer, PromptRenderPlugin};

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
        Self::minimal()
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
            symbol: Some(self.config.symbol.clone().unwrap_or_else(|| "> ".to_string())),
            buffer: String::new(),
        });
        app.insert_resource(self.config.clone());
        app.add_plugins(PromptInputPlugin);
        app.add_plugins(PromptRenderPlugin { renderer: self.renderer.clone() });
        // InFrame mode enables in-frame logging integration
        if matches!(self.mode, PromptMode::InFrame) {
            app.add_plugins(InFrameLogPlugin);
        }
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

impl ReplPromptConfig {
    /// Minimal preset: single-line bar, no border, no colors, no hint.
    pub fn minimal() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            border: None,
            color: None,
            hint: None,
        }
    }

    /// Pretty preset: border, colors, and right-aligned hint enabled.
    pub fn pretty() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            border: Some(PromptBorderConfig::default()),
            color: Some(PromptColorConfig::default()),
            hint: Some(PromptHintConfig::default()),
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub buffer: String,
}

/// Visual configuration for the REPL prompt bar.
#[derive(Resource, Clone)]
pub struct ReplPromptConfig {
    /// Prompt symbol to display before the buffer.
    pub symbol: Option<String>,
    /// Draw a border and title around the prompt bar.
    pub border: Option<PromptBorderConfig>,
    /// Enable colorful styles for title/prompt/hints.
    pub color: Option<PromptColorConfig>,
    /// Show a right-aligned hint text.
    pub hint: Option<PromptHintConfig>,
}

impl Default for ReplPromptConfig {
    fn default() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            border: Some(PromptBorderConfig::default()),
            color: Some(PromptColorConfig::default()),
            hint: Some(PromptHintConfig::default()),
        }
    }
}

/// Border styling configuration (placeholder for future fields)
#[derive(Clone, Default)]
pub struct PromptBorderConfig;

/// Color styling configuration (placeholder for future fields)
#[derive(Clone, Default)]
pub struct PromptColorConfig;

/// Hint styling/behavior configuration (placeholder for future fields)
#[derive(Clone, Default)]
pub struct PromptHintConfig;
