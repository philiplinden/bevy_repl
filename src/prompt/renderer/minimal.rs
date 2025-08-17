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

        // Layout
        let left_area = prompt_area;
        let prompt_symbol = ctx.prompt.symbol.clone().unwrap_or_default();
        // Display columns, not bytes/chars
        let prompt_width = Span::raw(prompt_symbol.clone()).width() as u16;
        if left_area.width <= prompt_width { return; }
        let visible_width = left_area.width - prompt_width;

        // Buffer windowing
        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Render text
        let mut spans = Vec::with_capacity(2);
        spans.push(Span::from(prompt_symbol.clone()));
        spans.push(Span::from(visible_buf));
        f.render_widget(Paragraph::new(Line::from(spans)), left_area);

        // Cursor position
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, buffer, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
