use bevy::prelude::*;
use bevy_ratatui::{
    crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind},
    event::KeyEvent,
};
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
        app.add_systems(PostUpdate, toggle_repl);
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
    pub commands: HashMap<String, Box<dyn crate::parse::CommandParser>>,
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

pub fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

pub fn toggle_repl(mut repl: ResMut<Repl>, mut key_events: EventReader<KeyEvent>) {
    if let Some(key) = repl.toggle_key {
        for event in key_events.read() {
            if event.code == key && event.kind == CrosstermKeyEventKind::Press {
                info!(
                    "{} REPL",
                    if !repl.enabled { "Enabling" } else { "Disabling" }
                );
                if repl.enabled {
                    repl.enabled = false;
                } else {
                    repl.enabled = true;
                }
            }
        }
    }
}
