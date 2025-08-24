use bevy::prelude::*;
use bevy_ratatui::context::TerminalContext;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Stdout, stdout};

use crate::repl::ReplLifecycleEvent;

pub struct ReplContextPlugin{
    backend: CrosstermBackend<Stdout>,
}

impl Plugin for ReplContextPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(manage_context);
        app.add_observer(cleanup_on_exit);
    }
}

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


#[derive(Resource, Deref, DerefMut, Debug)]
/// Terminal context used when `bevy_ratatui::RatatuiContext` is not available.
///
/// This keeps rendering on the main terminal screen (no alternate screen) using
/// `crossterm` via `ratatui`. It exists to provide a minimal, dependency-light
/// fallback so the REPL can render without the full ratatui stack.
pub struct FallbackTerminalContext(Terminal<CrosstermBackend<Stdout>>);

impl FallbackTerminalContext {
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

impl TerminalContext<CrosstermBackend<Stdout>> for FallbackTerminalContext {
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
fn manage_context(
    trigger: Trigger<ReplLifecycleEvent>,
    existing: Option<Res<FallbackTerminalContext>>,
    mut commands: Commands,
) {
    match trigger.event() {
        ReplLifecycleEvent::Enable => {
            if existing.is_none() {
                let Ok(terminal) = FallbackTerminalContext::init() else {
                    error!("Failed to initialize terminal context");
                    return;
                };
                commands.insert_resource(terminal);
                // Insert the guard so that any unexpected teardown restores raw mode.
                commands.insert_resource(RawModeGuard);
            }
        }
        ReplLifecycleEvent::Disable => {
            if existing.is_some() {
                let Ok(_) = FallbackTerminalContext::restore() else {
                    error!("Failed to remove terminal context");
                    return;
                };
                commands.remove_resource::<FallbackTerminalContext>();
                // Dropping the guard will also best-effort disable raw mode.
                commands.remove_resource::<RawModeGuard>();
            }
        }
    }
}

fn cleanup_on_exit(
    _exit: Trigger<AppExit>,
    mut commands: Commands,
    existing: Option<Res<FallbackTerminalContext>>,
) {
    // Ensure the resource is removed even if the lifecycle observer didn't run
    if existing.is_some() {
        let Ok(_) = FallbackTerminalContext::restore() else {
            error!("Failed to remove terminal context");
            return;
        };
        commands.remove_resource::<FallbackTerminalContext>();
        // Drop the guard on exit path as well.
        commands.remove_resource::<RawModeGuard>();
    }
}
