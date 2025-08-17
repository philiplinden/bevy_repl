use bevy::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};

/// Visual configuration for the REPL prompt bar.
#[derive(Resource, Clone)]
pub struct ReplPromptConfig {
    /// Prompt symbol to display before the buffer.
    pub symbol: Option<String>,
    /// Draw a block that surrounds the prompt.
    pub block: Option<Block<'static>>,
    /// Enable colorful styles for title/prompt/hints.
    pub style: Option<Style>,
    /// Show a right-aligned hint text.
    pub hint: Option<String>,
}

impl Default for ReplPromptConfig {
    fn default() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            style: None,
            block: None,
            hint: None,
        }
    }
}

impl ReplPromptConfig {
    /// Simple preset: single-line bar, no border, no colors, no hint.
    pub fn minimal() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            block: None,
            style: None,
            hint: None,
        }
    }

    pub fn simple() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            block: None,
            style: Some(Style::default().fg(Color::Yellow)),
            hint: None,
        }
    }

    /// Pretty preset: border, colors, and right-aligned hint enabled.
    pub fn pretty() -> Self {
        Self {
            symbol: Some("> ".to_string()),
            block: Some(Block::default().borders(Borders::ALL)),
            style: Some(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            hint: Some("Enter to run • Esc to clear".to_string()),
        }
    }
}
