use bevy::prelude::*;
use bevy_ratatui::{
    context::DefaultContext,
};

use crate::repl::ReplLifecycleEvent;

use bevy_ratatui::{
    event::EventPlugin,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    translation::TranslationPlugin,
};

/// Minimal Ratatui plugin group: replicates the default Ratatui plugin group
/// but without the alternate screen.
pub struct StdoutRatatuiPlugins;

impl PluginGroup for StdoutRatatuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EventPlugin::default())
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(TranslationPlugin)
    }
}

pub struct ReplContextPlugin;

impl Default for ReplContextPlugin {
    fn default() -> Self {
        Self
    }
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

/// A minimal, dependency-light fallback so the REPL can render without the full
/// ratatui stack. This keeps rendering on the main terminal screen (no
/// alternate screen) using `crossterm` via `ratatui`.
pub struct ReplContext;

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct RatatuiContext(pub DefaultContext);

impl Drop for RatatuiContext {
    fn drop(&mut self) {
        if let Err(err) = DefaultContext::restore() {
            eprintln!("Failed to restore terminal: {}", err);
        }
    }
}

impl RatatuiContext {
    pub fn init() -> Result<Self> {
        Ok(Self(DefaultContext::init()?))
    }

    pub fn restore() -> Result {
        DefaultContext::restore()
    }
}

/// Manage the terminal context on lifecycle events (startup/shutdown) to ensure
/// that the terminal context is set up and restored properly (and avoid
/// breaking the terminal after the app exits).
fn manage_context(
    trigger: Trigger<ReplLifecycleEvent>,
    context: Option<ResMut<RatatuiContext>>,
    mut commands: Commands,
) {
    match trigger.event() {
        ReplLifecycleEvent::Enable => {
            if context.is_none() {
                let Ok(terminal) = RatatuiContext::init() else {
                    error!("Failed to initialize terminal context");
                    return;
                };
                commands.insert_resource(terminal);
                // Insert the guard so that any unexpected teardown restores raw mode.
                commands.insert_resource(RawModeGuard);
            }
        }
        ReplLifecycleEvent::Disable => {
            if context.is_some() {
                let Ok(_) = RatatuiContext::restore() else {
                    error!("Failed to remove terminal context");
                    return;
                };
                commands.remove_resource::<RatatuiContext>();
                // Dropping the guard will also best-effort disable raw mode.
                commands.remove_resource::<RawModeGuard>();
            }
        }
    }
}

fn cleanup_on_exit(
    _exit: Trigger<AppExit>,
    mut commands: Commands,
    context: Option<ResMut<RatatuiContext>>,
) {
    // Ensure the resource is removed even if the lifecycle observer didn't run
    if context.is_some() {
        let Ok(_) = RatatuiContext::restore() else {
            error!("Failed to remove terminal context");
            return;
        };
        commands.remove_resource::<RatatuiContext>();
        // Drop the guard on exit path as well.
        commands.remove_resource::<RawModeGuard>();
    }
}
