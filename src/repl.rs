use bevy::prelude::*;
use bevy_ratatui::{context::TerminalContext, event::InputSet};
use std::io::{Stdout, stdout};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::collections::HashMap;

pub struct ReplPlugin {
    enable_on_startup: bool,
    toggle_key: Option<KeyCode>,
}

impl Default for ReplPlugin {
    fn default() -> Self {
        Self {
            enable_on_startup: true,
            toggle_key: Some(KeyCode::Backquote),
        }
    }
}

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Repl {
            enabled: self.enable_on_startup,
            toggle_key: self.toggle_key,
            ..default()
        });
        app.add_event::<ReplSubmitEvent>();
        app.add_event::<ReplBufferEvent>();
        app.add_event::<ReplToggleEvent>();
        app.add_observer(manage_context);
        app.add_observer(cleanup_on_exit);
        app.configure_sets(
            Update,
            (
                ReplSet::Toggle,
                ReplSet::Capture,
                ReplSet::Buffer,
                ReplSet::Render,
                ReplSet::Post,
            )
                .chain()
                .after(InputSet::EmitCrossterm)
                .before(InputSet::Post),
        );
    }
}

#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub toggle_key: Option<KeyCode>,
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
            toggle_key: Some(KeyCode::Backquote),
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
    pub fn toggle(&mut self) {
        if self.enabled {
            self.enabled = false;
            info!("REPL disabled");
        } else {
            self.enabled = true;
            info!("REPL enabled");
        }
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

#[derive(Event)]
pub enum ReplToggleEvent {
    Enable,
    Disable,
}

pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

/// Emit a toggle event if the REPL state has changed.
pub fn notify_toggle(
    repl: Res<Repl>,
    mut last_state: Local<bool>,
    mut toggle_events: EventWriter<ReplToggleEvent>,
) {
    if *last_state != repl.enabled {
        if repl.enabled {
            toggle_events.write(ReplToggleEvent::Enable);
        } else {
            toggle_events.write(ReplToggleEvent::Disable);
        }
        *last_state = repl.enabled;
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReplSet {
    /// Detect toggle via Bevy keyboard input
    Toggle,
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
pub struct ReplContext(Terminal<CrosstermBackend<Stdout>>);

impl ReplContext {
    /// Create a new ReplContext with a terminal and enable raw mode.
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

impl TerminalContext<CrosstermBackend<Stdout>> for ReplContext {
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

/// A system that sets up the terminal context. This runs when the Repl is
/// enabled to give it access to the terminal.
fn manage_context(
    trigger: Trigger<ReplToggleEvent>,
    existing: Option<Res<ReplContext>>,
    mut commands: Commands,
) {
    match trigger.event() {
        ReplToggleEvent::Enable => {
            if existing.is_none() {
                let Ok(terminal) = ReplContext::init() else {
                    error!("Failed to initialize terminal context");
                    return;
                };
                commands.insert_resource(terminal);
            }
        }
        ReplToggleEvent::Disable => {
            if existing.is_some() {
                let Ok(_) = ReplContext::restore() else {
                    error!("Failed to remove terminal context");
                    return;
                };
                commands.remove_resource::<ReplContext>();
            }
        }
    }
}

fn cleanup_on_exit(_exit: Trigger<AppExit>, mut commands: Commands) {
    let Ok(_) = ReplContext::restore() else {
        error!("Failed to remove terminal context");
        return;
    };
    commands.remove_resource::<ReplContext>();
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
