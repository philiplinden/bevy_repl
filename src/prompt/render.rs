use bevy::prelude::*;
use bevy_ratatui::crossterm::terminal;
use crate::repl::{Repl, ReplSet, repl_is_enabled};
use crate::prompt::ReplPrompt;

pub struct PromptRenderPlugin;

impl Plugin for PromptRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // When enabled, capture terminal input
                display_prompt
                    .in_set(ReplSet::Render)
                    .after(ReplSet::Buffer)
                    .run_if(repl_is_enabled),
            ),
        );
    }
}
/// System that displays the current input buffer at the bottom of the terminal
/// Runs whenever the Repl resource changes
pub(super) fn display_prompt(repl: Res<Repl>, prompt: Res<ReplPrompt>) {
    // Get terminal size
    let _ = match terminal::size() {
        Ok(size) => size,
        Err(_) => return, // If we can't get terminal size, skip rendering
    };

    // Display the prompt and current buffer
    let _prompt_text = if let Some(symbol) = prompt.symbol.clone() {
        format!("{}{}", symbol, repl.buffer)
    } else {
        repl.buffer.clone()
    };
}