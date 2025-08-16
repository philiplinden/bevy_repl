use super::ScrollRegionReadySet;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use bevy::prelude::*;
use bevy_ratatui::crossterm::terminal;
use std::io::{Write, stdout};

use super::helpers::{bottom_bar_area, buffer_window, cursor_position};
use super::{PromptRenderer, RenderCtx};
use crate::print::{printed_lines, set_scroll_region_info};
use crate::prompt::ReplPromptConfig;
use crate::repl::{Repl, ReplSet};

/// Return whether border is enabled under pretty feature.
pub fn border_on(cfg: &ReplPromptConfig) -> bool {
    cfg.border.is_some()
}

pub struct ScrollRegionPlugin;

impl Plugin for ScrollRegionPlugin {
    fn build(&self, app: &mut App) {
        // Ensure region is set early (before any PostStartup prints)
        app.add_systems(Startup, manage_pretty_scroll_region);
        // Run once in PostStartup too, in the labeled set, to catch cases where
        // terminal size isn't ready at Startup and to provide ordering guarantees.
        app.add_systems(
            PostStartup,
            manage_pretty_scroll_region.in_set(ScrollRegionReadySet),
        );
        app.add_systems(
            Update,
            (manage_pretty_scroll_region
                .in_set(ReplSet::Render)
                .in_set(ReplSet::All)
                .after(ReplSet::Buffer)
                .before(super::display_prompt),),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollRegionState {
    pub enabled: bool,
    pub height: u16,
    pub reserved_lines: u16,
}

/// Ensure the terminal scroll region reserves the bottom prompt area so that
/// stdout/logs scroll above the REPL prompt instead of overwriting it.
fn manage_pretty_scroll_region(
    repl: Res<Repl>,
    visuals: Option<Res<ReplPromptConfig>>,
    mut last: Local<Option<ScrollRegionState>>,
) {
    // Determine desired reserved lines for the prompt area: pretty uses a border (3 lines).
    let vis = visuals.map(|v| v.clone()).unwrap_or_default();
    let border_on = vis.border.is_some();
    let reserved_lines: u16 = if repl.enabled && border_on { 3 } else { 0 };

    // Read terminal size; if unavailable, do nothing
    let Ok((_w, h)) = terminal::size() else {
        return;
    };

    let desired = ScrollRegionState {
        enabled: repl.enabled,
        height: h,
        reserved_lines,
    };
    if last.as_ref() == Some(&desired) {
        return; // No change
    }

    let mut out = stdout();
    let prev_reserved = last.as_ref().map(|t| t.reserved_lines).unwrap_or(0);
    if reserved_lines == 0 {
        // If we never set a region before, do nothing (avoid touching terminal on minimal startup)
        if last.is_some() {
            // Reset to full region
            let _ = write!(out, "\x1B[r");
            // Publish reset so printers stop repositioning
            set_scroll_region_info(h, 0);
        }
    } else {
        // DECSTBM: ESC[{top};{bottom}r with 1-based coordinates
        // Reserve `reserved_lines` at the bottom => bottom = h - reserved_lines
        let bottom = h.saturating_sub(reserved_lines);
        let _ = write!(out, "\x1B[1;{}r", bottom);
        set_scroll_region_info(h, reserved_lines);
        scroll_reserved_region_up(
            &mut out,
            bottom,
            reserved_lines,
            prev_reserved,
            printed_lines(),
        );
    }
    let _ = out.flush();

    *last = Some(desired);
}

/// Scroll the reserved region up by emitting newlines at the last scrollable line.
///
/// This is used when the scroll region is first enabled (or transitioning from 0),
/// to ensure that the prompt area at the bottom is clear and that output appears
/// above the reserved region. This is generally more predictable across terminals
/// than using CSI S (scroll up).
///
/// # Arguments
/// * `out` - The output stream to write terminal escape codes to.
/// * `bottom` - The 1-based row number of the last scrollable line.
/// * `reserved_lines` - The number of lines reserved at the bottom.
/// * `prev_reserved` - The previous number of reserved lines.
/// * `printed_lines` - The number of lines already printed to the terminal.
fn scroll_reserved_region_up(
    out: &mut std::io::Stdout,
    bottom: u16,
    reserved_lines: u16,
    prev_reserved: u16,
    printed_lines: usize,
) {
    if prev_reserved == 0 && printed_lines > 0 {
        // Move to last scrollable line (1-based row: bottom)
        let _ = write!(out, "\x1B[{};1H", bottom);
        for _ in 0..reserved_lines {
            let _ = write!(out, "\n");
        }
    }
}

/// Compute full bar height based on border state.
pub fn bar_height(border_on: bool) -> u16 {
    if border_on { 3 } else { 1 }
}

/// Build the outer bordered block for the REPL title with optional color.
pub fn build_block(cfg: &ReplPromptConfig) -> Block<'static> {
    let title_style = if cfg.color.is_some() {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
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
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: 1,
        }
    } else {
        Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        }
    }
}

/// Optional right-aligned hint text to display with pretty feature.
pub fn hint_text(cfg: &ReplPromptConfig) -> Option<&'static str> {
    if cfg.hint.is_some() {
        Some("Enter to run • Esc to clear")
    } else {
        None
    }
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
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
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
        if ctx.area.height < height {
            return;
        }

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
        if visible_width == 0 {
            return;
        }

        // Buffer windowing to keep cursor visible
        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Left area and render
        let left_area = Rect {
            x: inner.x,
            y: inner.y,
            width: left_width,
            height: 1,
        };
        let mut spans = Vec::with_capacity(2);
        if !prompt_symbol.is_empty() {
            spans.push(Span::styled(prompt_symbol, prompt_style(visuals)));
        }
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
                let hint_para = Paragraph::new(Line::from(vec![Span::styled(
                    h.to_string(),
                    hint_style(visuals),
                )]));
                f.render_widget(hint_para, hint_area);
            }
        }

        // Cursor
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
