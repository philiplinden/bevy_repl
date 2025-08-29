use bevy::prelude::*;
use bevy_ratatui::event::InputSet;

use std::collections::HashMap;

/// A Bevy plugin that provides a Read-Eval-Print Loop (REPL) interface for interactive command input.
///
/// # Purpose
/// The `ReplPlugin` enables a REPL terminal within your Bevy application, allowing users to enter commands and interact with the app at runtime.
///
/// # Configuration Options
/// - `enable_on_startup`: Determines whether the REPL is enabled when the app starts.
///   - Use [`ReplPlugin::enabled()`] to start enabled (default).
///   - Use [`ReplPlugin::disabled()`] to start disabled.
///   - Use [`ReplPlugin::with_enabled(bool)`] for explicit control.
///
/// # Usage
/// Add the plugin to your Bevy app:
/// ```
/// use your_crate::ReplPlugin;
/// App::new().add_plugin(ReplPlugin::enabled());
/// ```
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
        app.add_systems(Startup, emit_enable_if_enabled);
        app.add_observer(on_app_exit_emit_disable);
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
        );
        // Wrapper set to anchor all REPL systems at the end of the ratatui set.
        // All of the REPL sets only run when the REPL is enabled.
        app.configure_sets(
            Update,
            ReplSet::All
                .in_set(InputSet::Post)
                .run_if(repl_is_enabled),
        );
    }
}

#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub buffer: String,
    pub cursor_pos: usize,
    pub commands: HashMap<String, Box<dyn crate::command::CommandParser>>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer: String::new(),
            cursor_pos: 0,
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

#[derive(Event)]
pub enum ReplLifecycleEvent {
    Enable,
    Disable,
}

// TODO: someday this could be a system triggered by state transitions
fn emit_enable_if_enabled(repl: Res<Repl>, mut writer: EventWriter<ReplLifecycleEvent>) {
    if repl.enabled {
        writer.write(ReplLifecycleEvent::Enable);
    }
}

fn on_app_exit_emit_disable(_exit: Trigger<AppExit>, mut writer: EventWriter<ReplLifecycleEvent>) {
    writer.write(ReplLifecycleEvent::Disable);
}

#[derive(Event, Debug)]
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

#[derive(Event, Debug)]
pub struct ReplSubmitEvent(pub String);
