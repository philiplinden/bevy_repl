use bevy::prelude::*;
use bevy_ratatui::context::TerminalContext;
use bevy_ratatui::{
    crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind},
    event::KeyEvent,
};
use std::io::{Stdout, stdout};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::collections::HashMap;

pub struct ReplPlugin {
    enable_on_startup: bool,
    toggle_key: Option<CrosstermKeyCode>,
}

impl Default for ReplPlugin {
    fn default() -> Self {
        Self {
            enable_on_startup: true,
            toggle_key: Some(CrosstermKeyCode::Char('`')),
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
        app.add_event::<ReplToggleEvent>();
        app.add_systems(PostUpdate, toggle_repl);
        app.add_observer(manage_context);
        app.add_observer(cleanup_on_exit);
    }
}

#[derive(Resource)]
pub struct Repl {
    pub enabled: bool,
    pub toggle_key: Option<CrosstermKeyCode>,
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
            toggle_key: Some(CrosstermKeyCode::Char('`')),
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

#[derive(Event)]
pub enum ReplToggleEvent {
    Enable,
    Disable,
}

pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

pub fn toggle_repl(
    mut repl: ResMut<Repl>,
    mut key_events: EventReader<KeyEvent>,
    mut toggle_events: EventWriter<ReplToggleEvent>,
) {
    if let Some(key) = repl.toggle_key {
        for event in key_events.read() {
            if event.code == key && event.kind == CrosstermKeyEventKind::Press {
                info!(
                    "{} REPL",
                    if !repl.enabled {
                        "Enabling"
                    } else {
                        "Disabling"
                    }
                );
                if repl.enabled {
                    toggle_events.write(ReplToggleEvent::Disable);
                    repl.enabled = false;
                } else {
                    toggle_events.write(ReplToggleEvent::Enable);
                    repl.enabled = true;
                }
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct ReplContext(Terminal<CrosstermBackend<Stdout>>);

impl TerminalContext<CrosstermBackend<Stdout>> for ReplContext {
    fn init() -> Result<Self> {
        let stdout = stdout();
        // Enable raw mode but stay in main screen
        bevy_ratatui::crossterm::terminal::enable_raw_mode().unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        Ok(Self(terminal))
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
fn manage_context(trigger: Trigger<ReplToggleEvent>, mut commands: Commands) {
    match trigger.event() {
        ReplToggleEvent::Enable => {
            let Ok(terminal) = ReplContext::init() else {
                error!("Failed to initialize terminal context");
                return;
            };
            commands.insert_resource(terminal);
        }
        ReplToggleEvent::Disable => {
            let Ok(_) = ReplContext::restore() else {
                error!("Failed to remove terminal context");
                return;
            };
            commands.remove_resource::<ReplContext>();
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
