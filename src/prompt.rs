use bevy::{app::Plugin, input::keyboard::KeyboardInput, prelude::*};
use bevy_ratatui::{
    crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind},
    event::KeyEvent,
};

pub struct PromptPlugin {
    /// The prompt to display in the REPL console to the left of the input area.
    pub prompt: String,
    /// Toggle the REPL console on and off.
    pub toggle_key: Option<CrosstermKeyCode>,
    /// Enable the REPL on startup.
    pub enable_on_startup: bool,
    /// Enable a border around the REPL console.
    pub border: bool,
}

impl Default for PromptPlugin {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            toggle_key: Some(CrosstermKeyCode::Char('`')),
            enable_on_startup: true,
            border: true,
        }
    }
}

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Repl {
            enabled: self.enable_on_startup,
            prompt: self.prompt.clone(),
            toggle_key: self.toggle_key,
        });
        app.init_resource::<ReplInput>();
        app.add_systems(Update, toggle_repl);
        app.add_systems(
            Update,
            (buffer_input, print_repl_input).chain().run_if(repl_is_enabled),
        );
    }
}

#[derive(Resource)]
pub struct Repl {
    enabled: bool,
    prompt: String,
    toggle_key: Option<CrosstermKeyCode>,
}

fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

fn toggle_repl(mut repl: ResMut<Repl>, mut key_events: EventReader<KeyEvent>) {
    if let Some(key) = repl.toggle_key {
        for event in key_events.read() {
            if event.code == key && event.kind == CrosstermKeyEventKind::Press {
                repl.enabled = !repl.enabled;
                info!(
                    "Repl is now {}",
                    if repl.enabled { "enabled" } else { "disabled" }
                );
            }
        }
    }
}

fn print_repl_input(repl_input: Res<ReplInput>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Enter) {
        info!("Repl input: {}", repl_input.buffer);
    }
}

#[derive(Resource)]
pub struct ReplInput {
    buffer: String,
    cursor_pos: usize,
}

impl Default for ReplInput {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            cursor_pos: 0,
        }
    }
}

fn buffer_input(
    mut repl_input: ResMut<ReplInput>,
    mut crossterm_key_events: EventReader<KeyEvent>,
) {
    for event in crossterm_key_events.read() {
        if let CrosstermKeyCode::Char(c) = event.code && event.kind == CrosstermKeyEventKind::Press {
            repl_input.buffer.push_str(&c.to_string());
            repl_input.cursor_pos += 1;
        }
        if let CrosstermKeyCode::Backspace = event.code && event.kind == CrosstermKeyEventKind::Press {
            repl_input.buffer.pop();
            repl_input.cursor_pos -= 1;
        }
    }
}
