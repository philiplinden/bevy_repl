use bevy::prelude::*;
use bevy_ratatui::{context::TerminalContext, event::InputSet};
use std::io::{Stdout, stdout};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::collections::HashMap;

pub struct ReplPlugin {
    enable_on_startup: bool,
}

impl Default for ReplPlugin {
    fn default() -> Self {
        Self {
            enable_on_startup: true,
        }
    }
}

impl ReplPlugin {
    /// Create a REPL plugin that starts enabled (default).
    pub fn enabled() -> Self {
        Self { enable_on_startup: true }
    }

    /// Create a REPL plugin that starts disabled (no runtime toggle in v1).
    pub fn disabled() -> Self {
        Self { enable_on_startup: false }
    }

    /// Configure whether the REPL starts enabled.
    pub fn with_enabled(enabled: bool) -> Self {
        Self { enable_on_startup: enabled }
    }
}

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Repl {
            enabled: self.enable_on_startup,
            ..default()
        });
        app.add_event::<ReplSubmitEvent>();
        app.add_event::<ReplBufferEvent>();
        // Internal lifecycle event to manage terminal context without runtime toggle
        app.add_event::<ReplLifecycleEvent>();
        app.add_observer(manage_context);
        app.add_systems(Startup, emit_enable_if_enabled);
        app.add_observer(on_app_exit_emit_disable);
        app.add_observer(cleanup_on_exit);
        app.configure_sets(
            Update,
            (
                ReplSet::Pre,
                ReplSet::Capture,
                ReplSet::Buffer,
                ReplSet::Render,
                ReplSet::Post,
            )
                .chain()
                .after(InputSet::EmitCrossterm)
                .before(InputSet::Post),
        );
        // Wrapper set to anchor all REPL systems between ratatui input emission and post-processing
        app.configure_sets(
            Update,
            ReplSet::All
                .after(InputSet::EmitCrossterm)
                .before(InputSet::Post)
                .run_if(repl_is_enabled),
        );
    }
}

#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub buffer: String,
    pub cursor_pos: usize,
    pub history: Vec<String>,
    pub history_index: usize,
    pub history_enabled: bool,
    pub commands: HashMap<String, Box<dyn crate::command::CommandParser>>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer: String::new(),
            cursor_pos: 0,
            history: Vec::new(),
            history_index: 0,
            history_enabled: true,
            commands: HashMap::new(),
        }
    }
}

impl Repl {
    pub fn drain_buffer(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.clear_buffer();
        buffer
    }
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.cursor_pos = 0;
    }
    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.buffer.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }
    pub fn delete(&mut self) {
        if self.cursor_pos < self.buffer.len() {
            self.buffer.remove(self.cursor_pos);
        }
    }
    pub fn left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }
    pub fn right(&mut self) {
        if self.cursor_pos < self.buffer.len() {
            self.cursor_pos += 1;
        }
    }
    pub fn home(&mut self) {
        self.cursor_pos = 0;
    }
    pub fn end(&mut self) {
        self.cursor_pos = self.buffer.len();
    }
    pub fn insert(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }
}

pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReplSet {
    /// Wrapper for all REPL systems to allow global ordering and run conditions
    All,
    /// Pre stage for consuming/forwarding behavior
    Pre,
    /// Read terminal key events (when enabled)
    Capture,
    /// Update REPL buffer state from captured input
    Buffer,
    /// Render the prompt / UI
    Render,
    /// Post stage for consuming/forwarding behavior
    Post,
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

#[derive(Event)]
enum ReplLifecycleEvent {
    Enable,
    Disable,
}

fn emit_enable_if_enabled(repl: Res<Repl>, mut writer: EventWriter<ReplLifecycleEvent>) {
    if repl.enabled {
        writer.write(ReplLifecycleEvent::Enable);
    }
}

fn on_app_exit_emit_disable(_exit: Trigger<AppExit>, mut writer: EventWriter<ReplLifecycleEvent>) {
    writer.write(ReplLifecycleEvent::Disable);
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
            }
        }
        ReplLifecycleEvent::Disable => {
            if existing.is_some() {
                let Ok(_) = FallbackTerminalContext::restore() else {
                    error!("Failed to remove terminal context");
                    return;
                };
                commands.remove_resource::<FallbackTerminalContext>();
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
    }
}

#[derive(Event)]
pub enum ReplBufferEvent {
    Insert(char),
    Backspace,
    Delete,
    MoveLeft,
    MoveRight,
    JumpToStart,
    JumpToEnd,
    Clear,
    Submit,
}

#[derive(Event)]
pub struct ReplSubmitEvent(pub String);
