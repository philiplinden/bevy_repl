pub mod input;
pub mod render;
pub mod key_events;
pub mod helpers;
#[cfg(feature = "pretty")]
pub mod pretty;

use bevy::prelude::*;

use crate::repl::{ReplSet};
use self::input::PromptInputPlugin;
use self::render::PromptRenderPlugin;
use self::key_events::block_keyboard_input_forwarding;

#[derive(Resource, Clone)]
pub struct PromptPlugin {
    pub config: ReplPromptConfig,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self {
            config: ReplPromptConfig::default(),
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplPrompt {
            symbol: Some(self.config.symbol.clone().unwrap_or_else(|| "> ".to_string())),
            buffer: String::new(),
        });
        // Visual configuration resource
        app.insert_resource(self.config.clone());
        // Compose prompt-related plugins
        app.add_plugins((PromptInputPlugin, PromptRenderPlugin));
        app.add_systems(
            Update,
            (
                // When enabled, capture terminal input
                // Render prompt/UI
                // Finally block forwarding while enabled, after render and toggle
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
