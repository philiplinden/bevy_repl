use bevy::prelude::*;
use bevy_ratatui::{
    crossterm::{
        ExecutableCommand, cursor,
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    error::ErrorPlugin,
    event::EventPlugin,
    kitty::{KittyEnabled, KittyPlugin},
    translation::TranslationPlugin,
};
use color_eyre::{
    self,
    config::{EyreHook, HookBuilder, PanicHook},
    eyre,
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Stdout, stdout};
use std::panic;

/// The plugin behaves like a [`RatatuiContext`] but for [`ReplContext`]. It
/// adds the [`ReplContext`] resource to the bevy application.
pub struct ReplContextPlugin;

impl Plugin for ReplContextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // We can use the regular bevy_ratatui events plugin because it doesn't
        // manage the RatatuiContext resource.
        if !app.is_plugin_added::<EventPlugin>() {
            app.add_plugins(EventPlugin::default());
        }
        // Enable kitty keyboard protocol to receive modifiers with special keys
        // like Enter where the terminal supports it (needed for Ctrl+Enter).
        if !app.is_plugin_added::<KittyPlugin>() {
            app.add_plugins(KittyPlugin);
        }
        // We can use the regular bevy_ratatui translation plugin because it
        // doesn't manage the RatatuiContext resource.
        if !app.is_plugin_added::<TranslationPlugin>() {
            app.add_plugins(TranslationPlugin);
        }
        // We are incompatible with bevy_ratatui's ErrorPlugin. If it is added,
        // prefer to use theirs.
        if !app.is_plugin_added::<ErrorPlugin>() {
            app.add_systems(Startup, error_setup);
        }
        // Replicates the bevy_ratatui ContextPlugin
        app.add_systems(Startup, context_setup);
        // Replicates the bevy_ratatui CleanupPlugin
        app.add_observer(context_cleanup);
    }
}

/// Behaves like a [`RatatuiContext`] but uses the main terminal screen (no
/// alternate screen) using `crossterm` via `ratatui`. Use this as you would use
/// a [`RatatuiContext`].
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct ReplContext(Terminal<CrosstermBackend<Stdout>>);

impl Drop for ReplContext {
    fn drop(&mut self) {
        if let Err(err) = ReplContext::restore() {
            eprintln!("Failed to restore terminal: {}", err);
        }
    }
}

impl ReplContext {
    fn init() -> Result<Self> {
        let stdout = stdout();
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }

    fn restore() -> Result<()> {
        let mut stdout = stdout();
        stdout.execute(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }
}

/// A startup system that sets up the terminal context.
pub fn context_setup(mut commands: Commands) -> Result {
    let terminal = ReplContext::init()?;
    commands.insert_resource(terminal);

    Ok(())
}

/// Equivalent to what [`CleanupPlugin`] does, but for the REPL context. (The
/// regular cleanup plugin will remove [`RatatuiContext`] resources but not
/// [`ReplContext`] so we have to do it ourselves.)
fn context_cleanup(_trigger: Trigger<AppExit>, mut commands: Commands) {
    commands.remove_resource::<KittyEnabled>();
    commands.remove_resource::<ReplContext>();
}

/// Installs hooks for panic and error handling. This is a ripoff of the
/// bevy_ratatui [`ErrorPlugin`].
///
/// Makes the app resilient to panics and errors by restoring the terminal
/// before printing the panic or error message. This prevents error messages
/// from being messed up by the terminal state.
pub fn error_setup() -> Result {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    set_panic_hook(panic_hook);
    set_error_hook(eyre_hook)?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
fn set_panic_hook(panic_hook: PanicHook) {
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = ReplContext::restore();
        panic_hook(panic_info);
    }));
}

/// Install an error hook that restores the terminal before printing the error.
fn set_error_hook(eyre_hook: EyreHook) -> Result {
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        let _ = ReplContext::restore();
        eyre_hook(error)
    }))?;

    Ok(())
}
