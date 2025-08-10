use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::prompt::ReplPromptConfig;
use super::{PromptRenderer, RenderCtx};
use super::helpers::{bottom_bar_area, buffer_window, cursor_position};

/// Return whether border is enabled under pretty feature.
pub fn border_on(cfg: &ReplPromptConfig) -> bool { cfg.border.is_some() }

/// Compute full bar height based on border state.
pub fn bar_height(border_on: bool) -> u16 { if border_on { 3 } else { 1 } }

/// Build the outer bordered block for the REPL title with optional color.
pub fn build_block(cfg: &ReplPromptConfig) -> Block<'static> {
    let title_style = if cfg.color.is_some() {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    Block::default()
        .borders(Borders::ALL)
        .title(" REPL ")
        .title_style(title_style)
}

/// Compute the inner rect (content line) given the outer area and border flag.
pub fn inner_rect(area: Rect, border_on: bool) -> Rect {
    if border_on {
        Rect { x: area.x + 1, y: area.y + 1, width: area.width.saturating_sub(2), height: 1 }
    } else {
        Rect { x: area.x, y: area.y, width: area.width, height: 1 }
    }
}

/// Optional right-aligned hint text to display with pretty feature.
pub fn hint_text(cfg: &ReplPromptConfig) -> Option<&'static str> {
    if cfg.hint.is_some() { Some("Enter to run • Esc to clear") } else { None }
}

/// Hint style based on cfg.color.
pub fn hint_style(cfg: &ReplPromptConfig) -> Style {
    if cfg.color.is_some() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    }
}

/// Prompt symbol style based on cfg.color.
pub fn prompt_style(cfg: &ReplPromptConfig) -> Style {
    if cfg.color.is_some() {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    }
}

/// Pretty renderer strategy using the helpers above
pub struct PrettyRenderer;

impl PromptRenderer for PrettyRenderer {
    fn render(&self, f: &mut ratatui::Frame<'_>, ctx: &RenderCtx) {
        let visuals = ctx.visuals;

        // Determine total height: 1 line content + optional borders
        let border_on = border_on(visuals);
        let height = bar_height(border_on);
        if ctx.area.height < height { return; }

        // Bottom bar area
        let area = bottom_bar_area(ctx.area, height);

        // Optional bordered block
        if border_on {
            let block = build_block(visuals);
            f.render_widget(block, area);
        }

        // Inner content line
        let inner = inner_rect(area, border_on);

        // Hint
        let hint_text_opt = hint_text(visuals);
        let hint_width = hint_text_opt.map(|h| h.len() as u16).unwrap_or(0);
        let spacing = if hint_width > 0 { 1 } else { 0 };

        // Left width and prompt symbol
        let left_width = inner.width.saturating_sub(hint_width + spacing);
        let prompt_symbol = ctx.prompt.symbol.clone().unwrap_or_default();
        let prompt_width = prompt_symbol.len() as u16;
        let visible_width = left_width.saturating_sub(prompt_width);
        if visible_width == 0 { return; }

        // Buffer windowing to keep cursor visible
        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Left area and render
        let left_area = Rect { x: inner.x, y: inner.y, width: left_width, height: 1 };
        let mut spans = Vec::with_capacity(2);
        if !prompt_symbol.is_empty() { spans.push(Span::styled(prompt_symbol, prompt_style(visuals))); }
        spans.push(Span::raw(visible_buf));
        f.render_widget(Paragraph::new(Line::from(spans)), left_area);

        // Hint on the right
        if let Some(h) = hint_text_opt {
            if hint_width > 0 && left_width + spacing < inner.width {
                let hint_area = Rect {
                    x: inner.x + left_width + spacing,
                    y: inner.y,
                    width: inner.width - left_width - spacing,
                    height: 1,
                };
                let hint_para = Paragraph::new(Line::from(vec![Span::styled(h.to_string(), hint_style(visuals))]));
                f.render_widget(hint_para, hint_area);
            }
        }

        // Cursor
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
