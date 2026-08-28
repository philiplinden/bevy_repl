use crate::command::CommandParser;

use bevy::prelude::*;
use crossterm::event::KeyCode;
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
///
/// # Note
/// For a complete batteries-included REPL experience, consider using the [`ReplPlugins`] group.
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
        Self {
            enable_on_startup: true,
        }
    }

    /// Create a REPL plugin that starts disabled (no runtime toggle in v1).
    pub fn disabled() -> Self {
        Self {
            enable_on_startup: false,
        }
    }

    /// Configure whether the REPL starts enabled.
    pub fn with_enabled(enabled: bool) -> Self {
        Self {
            enable_on_startup: enabled,
        }
    }
}

pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Repl {
            enabled: self.enable_on_startup,
            ..default()
        });
        app.add_message::<ReplSubmitEvent>();
        app.add_message::<ReplBufferEvent>();
        app.add_message::<ReplLifecycleEvent>();
        app.add_systems(Startup, emit_enable_if_enabled);
        app.add_systems(Last, on_app_exit_emit_disable);
        app.configure_sets(
            Update,
            (
                ReplSet::Pre,
                ReplSet::Capture,
                ReplSet::Buffer,
                ReplSet::Parse,
                ReplSet::Render,
                ReplSet::Post,
            )
                .chain(),
        );
        // Wrapper set to anchor all REPL systems at the end of the PreUpdate set.
        // All of the REPL sets only run when the REPL is enabled.
        app.configure_sets(
            Update,
            ReplSet::All.in_set(PreUpdate).run_if(repl_is_enabled),
        );
    }
}

/// The REPL resource holds the state of the REPL, including the buffer, cursor position, and commands.
/// Core functions for interacting with the REPL itself and managing the buffer are implemented as methods on this resource.
/// Custom app commands should be registered with the [`register_command`] method, not the [`Repl`] resource itself.
#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub prompt_symbol: String,
    pub buffer: String,
    pub cursor_pos: usize,
    pub toggle_key: Option<KeyCode>,
    pub commands: HashMap<String, Box<dyn CommandParser>>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            enabled: true,
            prompt_symbol: "> ".to_string(),
            buffer: String::new(),
            cursor_pos: 0,
            toggle_key: Some(KeyCode::Backquote),
            commands: HashMap::new(),
        }
    }
}

impl Repl {
    pub fn clear_terminal() -> std::io::Result<()> {
        use crossterm::{
            cursor::MoveTo,
            execute,
            terminal::{Clear, ClearType},
        };
        use std::io::stdout;

        let mut out = stdout();
        execute!(
            out,
            Clear(ClearType::All),
            Clear(ClearType::Purge), // Clears scrollback history if supported
            MoveTo(0, 0)
        )?;
        Ok(())
    }
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
    /// Parse commands from buffered REPL input
    Parse,
    /// Render the prompt / UI to the console
    Render,
    /// Post stage for consuming/forwarding behavior
    Post,
}

#[derive(Message, Debug, Clone)]
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

#[derive(Message, Debug, Clone)]
pub struct ReplSubmitEvent(pub String);

/// Event emitted when the REPL is enabled or disabled to notify other systems of the change.
#[derive(Message, Clone)]
pub enum ReplLifecycleEvent {
    Enable,
    Disable,
}

/// Function that emits a `ReplLifecycleEvent::Enable` message if the REPL is enabled.
///
/// The primary purpose of this function is to emit the `Enable` event when the plugin is initialized
/// to start up the REPL immediately. See also [`ReplPlugin.enabled_on_startup`].
fn emit_enable_if_enabled(repl: Res<Repl>, mut writer: MessageWriter<ReplLifecycleEvent>) {
    if repl.enabled {
        writer.write(ReplLifecycleEvent::Enable);
    }
}

/// Function that emits a `ReplLifecycleEvent::Disable` message when the application exits.
///
/// The primary purpose of this function is to trigger the event that notifies
/// other systems to clean up resources, stop the REPL, and restore the terminal
/// to its nominal state.
fn on_app_exit_emit_disable(
    mut exit: MessageReader<AppExit>,
    mut writer: MessageWriter<ReplLifecycleEvent>,
) {
    for _ in exit.read() {
        writer.write(ReplLifecycleEvent::Disable);
    }
}

pub fn handle_repl_lifecycle(
    mut reader: MessageReader<ReplLifecycleEvent>,
    mut repl: ResMut<Repl>,
) {
    for event in reader.read() {
        let should_enable = match event {
            ReplLifecycleEvent::Enable => true,
            ReplLifecycleEvent::Disable => false,
            ReplLifecycleEvent::Toggle => !repl.enabled,
        };

        if should_enable && !repl.enabled {
            repl.enabled = true;
            let _ = Repl::init_terminal();
        } else if !should_enable && repl.enabled {
            repl.enabled = false;
            repl.clear_buffer();
            let _ = Repl::restore_terminal();
        }
    }
}
