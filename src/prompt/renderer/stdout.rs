use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use super::{PromptRenderer, RenderCtx};
use super::helpers::{bottom_bar_area, buffer_window, cursor_position};

use bevy::prelude::*;
use bevy_ratatui::{context::TerminalContext};
use std::io::{Stdout, stdout};
use ratatui::{Terminal, backend::CrosstermBackend};
use crate::repl::ReplLifecycleEvent;

/// Stdout renderer: prints to stdout without a new terminal screen using a basic ratatui context
pub struct StdoutRenderer;
impl PromptRenderer for StdoutRenderer {
    fn render(&self, f: &mut Frame<'_>, ctx: &RenderCtx) {
        if ctx.area.height == 0 { return; }

        // Reserve bottom 1 line for the prompt
        let prompt_area = bottom_bar_area(ctx.area, 1);

        // Layout
        let left_area = prompt_area;
        let prompt_symbol = ctx
            .cfg
            .symbol
            .as_ref()
            .map(|s| s.text.clone())
            .unwrap_or_default();
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
    fn configure_context(&self, app: &mut App) {
        app.add_observer(manage_stdout_context);
        app.add_observer(cleanup_stdout_context_on_exit);
    }
}

#[derive(Resource, Deref, DerefMut, Debug)]
/// Terminal context used when `bevy_ratatui::RatatuiContext` is not available.
///
/// This keeps rendering on the main terminal screen (no alternate screen) using
/// `crossterm` via `ratatui`. It exists to provide a minimal, dependency-light
/// fallback so the REPL can render without the full ratatui stack.
pub struct StdoutTerminalContext(Terminal<CrosstermBackend<Stdout>>);

/// Guard resource that ensures terminal raw mode is disabled when dropped.
///
/// This complements `FallbackTerminalContext::restore()` and provides
/// a final line of defense during unwinding or unexpected teardown.
#[derive(Resource, Debug)]
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Idempotent, ignore errors; we just want to best-effort restore.
        let _ = bevy_ratatui::crossterm::terminal::disable_raw_mode();
    }
}

impl StdoutTerminalContext {
    /// Create a new `FallbackTerminalContext` with a terminal and enable raw mode.
    ///
    /// This is a workaround to initialize a `bevy_ratatui` terminal context
    /// without spawning an alternate screen.
    ///
    /// We have a separate method so that we can allow the user to initialize
    /// the context with their own terminal. The trait from `bevy_ratatui` does
    /// not allow the user to provide their own terminal and always creates a
    /// new one.
    ///
    /// This method is a simple change that sets up possible future
    /// functionality like using the REPL in a UI.
    pub fn with_terminal(terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<Self> {
        bevy_ratatui::crossterm::terminal::enable_raw_mode()?;
        Ok(Self(terminal))
    }
}

impl TerminalContext<CrosstermBackend<Stdout>> for StdoutTerminalContext {
    fn init() -> Result<Self> {
        let stdout = stdout();
        // Enable raw mode but stay in main screen
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Self::with_terminal(terminal)
    }

    fn restore() -> Result<()> {
        bevy_ratatui::crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
    fn configure_plugin_group(
        _group: &bevy_ratatui::RatatuiPlugins,
        builder: bevy::app::PluginGroupBuilder,
    ) -> bevy::app::PluginGroupBuilder {
        builder
    }
}

/// Manage the terminal context on lifecycle events (startup/shutdown).
/// If the REPL is using the Ratatui context, this is handled by the Ratatui plugin.
fn manage_stdout_context(
    trigger: Trigger<ReplLifecycleEvent>,
    existing: Option<Res<StdoutTerminalContext>>,
    mut commands: Commands,
) {
    match trigger.event() {
        ReplLifecycleEvent::Enable => {
            if existing.is_none() {
                let Ok(terminal) = StdoutTerminalContext::init() else {
                    error!("Failed to initialize stdout terminal context");
                    return;
                };
                commands.insert_resource(terminal);
                // Insert the guard so that any unexpected teardown restores raw mode.
                commands.insert_resource(RawModeGuard);
            }
        }
        ReplLifecycleEvent::Disable => {
            if existing.is_some() {
                let Ok(_) = StdoutTerminalContext::restore() else {
                    error!("Failed to remove terminal context");
                    return;
                };
                commands.remove_resource::<StdoutTerminalContext>();
                // Dropping the guard will also best-effort disable raw mode.
                commands.remove_resource::<RawModeGuard>();
            }
        }
    }
}

fn cleanup_stdout_context_on_exit(
    _exit: Trigger<AppExit>,
    mut commands: Commands,
    ctx: Option<Res<StdoutTerminalContext>>,
) {
    // Ensure the resource is removed even if the lifecycle observer didn't run
    if ctx.is_some() {
        let Ok(_) = StdoutTerminalContext::restore() else {
            error!("Failed to remove terminal context");
            return;
        };
        commands.remove_resource::<StdoutTerminalContext>();
        // Drop the guard on exit path as well.
        commands.remove_resource::<RawModeGuard>();
    }
}