use bevy::prelude::*;
use std::io::{stdout, Write};
use bevy_ratatui::crossterm::terminal;

use crate::prompt::ReplPromptConfig;
use crate::print::{set_scroll_region_info, printed_lines};
use crate::repl::{Repl, ReplSet};

pub struct ScrollRegionPlugin;

impl Plugin for ScrollRegionPlugin {
    fn build(&self, app: &mut App) {
        // Ensure region is set early (before any PostStartup prints)
        app.add_systems(Startup, manage_pretty_scroll_region);
        // Run once in PostStartup too, in the labeled set, to catch cases where
        // terminal size isn't ready at Startup and to provide ordering guarantees.
        app.add_systems(PostStartup, manage_pretty_scroll_region.in_set(ScrollRegionReadySet));
        app.add_systems(
            Update,
            (
                manage_pretty_scroll_region
                    .in_set(ReplSet::Render)
                    .in_set(ReplSet::All)
                    .after(ReplSet::Buffer)
                    .before(super::display_prompt),
            ),
        );

        // Expose the PostStartup ready set unconditionally so callers can order after it.
        app.configure_sets(PostStartup, ScrollRegionReadySet);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ScrollRegionReadySet;

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
    let Ok((_w, h)) = terminal::size() else { return };

    let desired = ScrollRegionState { enabled: repl.enabled, height: h, reserved_lines };
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
        scroll_reserved_region_up(&mut out, bottom, reserved_lines, prev_reserved, printed_lines());
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
