use crate::{command::CommandParser, keymap::ReplKeymap};

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
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy::log::LogPlugin>() {
            debug!("No global tracing subscriber exists yet!");
            info!("Configuring tracing layer for REPL...");
            let _ = tracing_subscriber::registry()
                .with(crate::tracing::repl_tracing_layer())
                .try_init();
        }
        app.insert_resource(Repl::default());
        app.add_message::<ReplSubmitEvent>();
        app.add_message::<ReplBufferEvent>();
        app.add_systems(Startup, init_repl);
        app.add_systems(Last, on_app_exit_emit_disable);
        app.configure_sets(
            Update,
            (
                ReplSet::Pre,
                ReplSet::Capture,
                ReplSet::Buffer,
                ReplSet::Parse,
                ReplSet::Print,
                ReplSet::Post,
            )
                .chain(),
        );
        // Wrapper set to anchor all REPL systems at the end of the PreUpdate set.
        // All of the REPL sets only run when the REPL is enabled.
        app.configure_sets(PreUpdate, ReplSet::All.run_if(repl_is_enabled));
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
    pub keymap: ReplKeymap,
    pub commands: HashMap<String, Box<dyn CommandParser>>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            enabled: false,
            prompt_symbol: "> ".to_string(),
            buffer: String::new(),
            cursor_pos: 0,
            keymap: ReplKeymap::default(),
            commands: HashMap::new(),
        }
    }
}

impl Repl {
    pub fn enable(&self) {
        self.enabled = true

    pub fn disable(&self) {
        self.enabled = false
    }
    pub fn toggle(&self) {
        self.enabled = !self.enabled
    }
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
    pub fn clear_to_start(&mut self) {
        if self.cursor_pos > 0 {
            self.buffer.drain(..self.cursor_pos);
            self.cursor_pos = 0;
        }
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

/// A simple helper function to control the `run_if` condition for the repl systems.
///
/// FIXME(2026-08-28): I'm not sure what happens if there is no Repl resource when calling this function.
pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReplSet {
    /// Global wrapper set to anchor all REPL systems and apply `run_if(repl_is_enabled)`
    All,
    /// Pre stage for consuming/forwarding behavior
    Pre,
    ///  Read keys from crossterm (when REPL is enabled)
    Capture,
    /// Update REPL buffer and cursor position from captured input
    Buffer,
    /// Parse and dispatch commands+args from buffered REPL input
    Parse,
    /// Output the prompt line to stdout
    Print,
    /// Post stage for input suppression / cleanup
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
    ClearToStart,
    ClearScreen,
    Submit,
}

#[derive(Event, Debug, Clone)]
pub struct ReplSubmitEvent(pub String);

/// Event emitted when the REPL is enabled or disabled to notify other systems of the change.
#[derive(Message, Clone)]
pub enum ReplLifecycleEvent {
    Enable,
    Disable,
    Toggle,
}
