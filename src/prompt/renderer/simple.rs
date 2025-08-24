
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use super::{RenderCtx, PromptRenderer};
use super::helpers::{bottom_bar_area, buffer_window, cursor_position};

/// Simple renderer: single line, no borders, no colors, no hints
pub struct SimpleRenderer;
impl PromptRenderer for SimpleRenderer {
    fn render(&self, f: &mut Frame<'_>, ctx: &RenderCtx) {
        // Always one line
        if ctx.area.height == 0 { return; }
        let area = bottom_bar_area(ctx.area, 1);

        // Layout
        let left_area = area;
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
