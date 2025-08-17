use bevy::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};

/// Visual configuration for the REPL prompt bar.
#[derive(Resource, Clone)]
pub struct ReplPromptConfig {
    /// Prompt symbol to display before the buffer.
    pub symbol: Option<PromptSymbol>,
    /// Draw a block that surrounds the prompt.
    pub block: Option<Block<'static>>,
    /// Show a right-aligned hint text.
    pub hint: Option<PromptHint>,
}

impl Default for ReplPromptConfig {
    fn default() -> Self {
        Self {
            symbol: Some(PromptSymbol::default()),
            block: None,
            hint: None,
        }
    }
}

impl ReplPromptConfig {
    /// Simple preset: single-line bar, no border, no colors, no hint.
    pub fn minimal() -> Self {
        Self {
            symbol: Some(PromptSymbol::default()),
            block: None,
            hint: None,
        }
    }

    pub fn simple() -> Self {
        Self {
            symbol: Some(PromptSymbol::default()),
            block: None,
            hint: None, 
        }
    }

    /// Pretty preset: border, colors, and right-aligned hint enabled.
    pub fn pretty() -> Self {
        Self {
            symbol: Some(PromptSymbol::default()),
            block: Some(Block::default().borders(Borders::ALL)),
            hint: Some(PromptHint::default()),
        }
    }
}

#[derive(Clone)]
pub struct PromptSymbol {
    pub text: String,
    pub style: Style,
}

impl Default for PromptSymbol {
    fn default() -> Self {
        Self {
            text: "> ".to_string(),
            style: Style::default().fg(Color::Yellow),
        }
    }
}

#[derive(Clone)]
pub struct PromptHint {
    pub text: String,
    pub style: Style,
}

impl Default for PromptHint {
    fn default() -> Self {
        Self {
            text: "Enter to run • Esc to clear".to_string(),
            style: Style::default().fg(Color::Gray),
        }
    }
}