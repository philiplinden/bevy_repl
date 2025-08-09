pub mod keymap;
pub mod input;
pub mod render;
pub mod key_events;

use bevy::prelude::*;

use crate::repl::{ReplSet, repl_is_enabled};
use self::input::PromptInputPlugin;
use self::render::PromptRenderPlugin;
use self::key_events::block_keyboard_input_forwarding;

#[derive(Resource, Clone)]
pub struct PromptPlugin {
    /// The prompt to display in the REPL console to the left of the input area.
    pub prompt: String,
    /// Enable a border around the REPL console.
    pub border: bool,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            border: true,
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplPrompt {
            symbol: Some(self.prompt.clone()),
            buffer: String::new(),
        });
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
                    .after(ReplSet::Render)
                    .run_if(repl_is_enabled),
            ),
        );
    }
}

#[derive(Resource, Default, Clone)]
pub struct ReplPrompt {
    pub symbol: Option<String>,
    pub buffer: String,
}
