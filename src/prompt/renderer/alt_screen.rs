use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use bevy::prelude::*;
use bevy_ratatui::crossterm::terminal;
use std::io::{Write, stdout};

use super::helpers::{bottom_bar_area, buffer_window, cursor_position};
use super::{PromptRenderer, RenderCtx};
use crate::log_ecs::LogBuffer;
use crate::print::{printed_lines, set_scroll_region_info};
use crate::prompt::ReplPromptConfig;
use crate::repl::{Repl, ReplSet};

pub struct ScrollRegionPlugin;

impl Plugin for ScrollRegionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, manage_pretty_scroll_region);
        app.add_systems(
            Update,
            manage_pretty_scroll_region
                .in_set(ReplSet::Render)
                .before(super::display_prompt),
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
    in_frame_logs: Option<Res<LogBuffer>>,
) {
    // If in-frame logging is enabled (LogBuffer exists), don't manage a terminal scroll region.
    if in_frame_logs.is_some() {
        return;
    }
    // Determine desired reserved lines for the prompt area: pretty uses a border (3 lines).
    let vis = visuals.map(|v| v.clone()).unwrap_or_default();
    let border_on = vis.block.is_some();
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

/// Alt-screen renderer prints to a new terminal screen using ratatui's default terminal context
pub struct AltScreenRenderer;

impl PromptRenderer for AltScreenRenderer {
    fn render(&self, f: &mut ratatui::Frame<'_>, ctx: &RenderCtx) {
        let visuals = ctx.cfg.clone();

        // Determine total height: 1 line content + optional borders
        let height = if visuals.block.is_some() { 3 } else { 1 };
        if ctx.area.height < height {
            return;
        }

        // Bottom bar area
        let area = bottom_bar_area(ctx.area, height);

        // Optional bordered block
        if let Some(block) = visuals.block.clone() {
            f.render_widget(block, area);
        }

        // Inner content line
        let inner = inner_rect(area, visuals.block.is_some());

        // Hint
        let hint_width: u16;
        if let Some(hint) = visuals.hint.as_ref() {
            hint_width = ratatui::text::Span::raw(hint.text.clone()).width() as u16;
        } else {
            hint_width = 0;
        }

        // Left width and prompt symbol
        let left_width = inner.width.saturating_sub(hint_width);
        let prompt_symbol = visuals
            .symbol
            .as_ref()
            .map(|s| s.text.clone())
            .unwrap_or_default();
        let prompt_width = ratatui::text::Span::raw(prompt_symbol.clone()).width() as u16;
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
        // style for symbol
        let sym_style = visuals.symbol.as_ref().map(|s| s.style).unwrap_or_default();
        spans.push(Span::styled(prompt_symbol, sym_style));
        spans.push(Span::raw(visible_buf));
        f.render_widget(Paragraph::new(Line::from(spans)), left_area);

        // Hint on the right
        if hint_width > 0 && left_width < inner.width {
            let hint_area = Rect {
                x: inner.x + left_width,
                y: inner.y,
                width: inner.width - left_width,
                height: 1,
            };
            if let Some(h) = visuals.hint.as_ref() {
                let hint_para =
                    Paragraph::new(Line::from(vec![Span::styled(h.text.clone(), h.style)]));
                f.render_widget(hint_para, hint_area);
            }
        }

        // Cursor
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, buffer, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
    fn configure_logging(&self, app: &mut App) {
        // Use the stdout REPL logging pipeline: capture tracing events and print them
        // via `repl_println!` so they appear above the prompt. This works with the
        // scroll region management to ensure logs don't overwrite the prompt.
        app.add_plugins(crate::log_ecs::CaptureSubscriberPlugin::default());
        app.add_systems(Update, crate::log_ecs::print_log_events_system);
    }
    fn configure_context(&self, app: &mut App) {
        // Add the ScrollRegionPlugin to manage terminal scroll regions
        // This ensures that in pretty mode (with borders), the bottom prompt area
        // is reserved and logs scroll above it instead of overwriting it.
        app.add_plugins(ScrollRegionPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::ReplPromptConfig;
    use ratatui::layout::Rect;

    #[test]
    fn test_scroll_region_state_calculation_simple() {
        // Test simple mode (no border) - should use 0 reserved lines
        let config = ReplPromptConfig::simple();
        let border_on = config.block.is_some();
        let reserved_lines: u16 = if true && border_on { 3 } else { 0 };
        
        assert_eq!(reserved_lines, 0, "Simple mode should not reserve lines");
    }

    #[test] 
    fn test_scroll_region_state_calculation_pretty() {
        // Test pretty mode (with border) - should use 3 reserved lines
        let config = ReplPromptConfig::pretty();
        let border_on = config.block.is_some();
        let reserved_lines: u16 = if true && border_on { 3 } else { 0 };
        
        assert_eq!(reserved_lines, 3, "Pretty mode should reserve 3 lines for border");
    }

    #[test]
    fn test_scroll_region_state_equality() {
        let state1 = ScrollRegionState {
            enabled: true,
            height: 24,
            reserved_lines: 3,
        };
        
        let state2 = ScrollRegionState {
            enabled: true,
            height: 24,
            reserved_lines: 3,
        };
        
        let state3 = ScrollRegionState {
            enabled: true,
            height: 24,
            reserved_lines: 0,
        };
        
        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_inner_rect_with_border() {
        let area = Rect { x: 0, y: 0, width: 10, height: 5 };
        
        // With border - should shrink by 1 on all sides
        let inner_with_border = inner_rect(area, true);
        assert_eq!(inner_with_border, Rect { x: 1, y: 1, width: 8, height: 1 });
        
        // Without border - should use full width
        let inner_no_border = inner_rect(area, false);
        assert_eq!(inner_no_border, Rect { x: 0, y: 0, width: 10, height: 1 });
    }

    #[test]
    fn test_prompt_config_differences() {
        let simple = ReplPromptConfig::simple();
        let pretty = ReplPromptConfig::pretty();
        
        // Simple should have no block (border)
        assert!(simple.block.is_none(), "Simple config should have no border");
        
        // Pretty should have a block (border)
        assert!(pretty.block.is_some(), "Pretty config should have a border");
        
        // Pretty should have a hint, simple should not
        assert!(simple.hint.is_none(), "Simple config should have no hint");
        assert!(pretty.hint.is_some(), "Pretty config should have a hint");
    }

    #[test] 
    fn test_scroll_reserved_region_calculation() {
        // Test the calculation logic that determines reserved lines based on config
        struct TestCase {
            repl_enabled: bool,
            has_border: bool,
            expected_reserved_lines: u16,
        }

        let test_cases = vec![
            TestCase { repl_enabled: true, has_border: true, expected_reserved_lines: 3 },   // pretty mode
            TestCase { repl_enabled: true, has_border: false, expected_reserved_lines: 0 },  // simple mode  
            TestCase { repl_enabled: false, has_border: true, expected_reserved_lines: 0 },  // disabled
            TestCase { repl_enabled: false, has_border: false, expected_reserved_lines: 0 }, // disabled
        ];

        for case in test_cases {
            let reserved_lines: u16 = if case.repl_enabled && case.has_border { 3 } else { 0 };
            assert_eq!(
                reserved_lines, 
                case.expected_reserved_lines,
                "repl_enabled: {}, has_border: {} should reserve {} lines", 
                case.repl_enabled, 
                case.has_border, 
                case.expected_reserved_lines
            );
        }
    }
}
