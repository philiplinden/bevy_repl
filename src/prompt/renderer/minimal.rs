use bevy::prelude::*;
use crate::log_ecs::InFrameLogPlugin;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use super::{RenderCtx, PromptRenderer};
use super::helpers::{bottom_bar_area, buffer_window, cursor_position};

/// Minimal renderer: single line, no borders, no colors, no hints
pub struct MinimalRenderer;
impl PromptRenderer for MinimalRenderer {
    fn render(&self, f: &mut Frame<'_>, ctx: &RenderCtx) {
        if ctx.area.height == 0 { return; }

        // Reserve bottom 1 line for the prompt
        let prompt_area = bottom_bar_area(ctx.area, 1);

        // Draw logs (if any) in the remaining area above the prompt, minimal styling
        if let Some(logs) = &ctx.logs {
            let log_height = ctx.area.height.saturating_sub(1);
            if log_height > 0 {
                let log_area = ratatui::layout::Rect {
                    x: ctx.area.x,
                    y: ctx.area.y,
                    width: ctx.area.width,
                    height: log_height,
                };
                // Take the last lines that fit vertically
                let count = logs.len() as u16;
                let take = count.min(log_height) as usize;
                let start = logs.len().saturating_sub(take);
                let lines: Vec<Line> = logs[start..]
                    .iter()
                    .map(|l| Line::from(format!("{:5} {}", l.level, l.message)))
                    .collect();
                f.render_widget(Paragraph::new(lines), log_area);
            }
        }

        // Layout
        let left_area = prompt_area;
        let prompt_symbol = ctx.prompt.symbol.clone().unwrap_or_default();
        let prompt_width = prompt_symbol.len() as u16;
        if left_area.width <= prompt_width { return; }
        let visible_width = left_area.width - prompt_width;

        // Buffer windowing
        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Render text
        let mut spans = Vec::with_capacity(2);
        if !prompt_symbol.is_empty() { spans.push(Span::raw(prompt_symbol)); }
        spans.push(Span::raw(visible_buf));
        f.render_widget(Paragraph::new(Line::from(spans)), left_area);

        // Cursor position
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
