use bevy::prelude::*;
use bevy_ratatui::{
    context::TerminalContext,
    cleanup::CleanupPlugin,
    error::ErrorPlugin,
    event::{EventPlugin, KeyEvent},
    translation::TranslationPlugin,
};
use bevy_ratatui::crossterm::event::{KeyCode as CrosstermKeyCode, KeyEventKind as CrosstermKeyEventKind};
use std::io::{Stdout, stdout};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use bevy::{
    app::{Plugin, PluginGroup, PluginGroupBuilder, Startup},
    input::keyboard::KeyboardInput,
};


pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CleanupPlugin)
            .add(ErrorPlugin)
            .add(EventPlugin::default())
            .add(TranslationPlugin)
            .add(ReplContextPlugin)
            .add(PromptPlugin::default())
    }
}

struct ReplContextPlugin;

impl Plugin for ReplContextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, context_setup);
    }
}

/// A startup system that sets up the terminal context.
pub fn context_setup(mut commands: Commands) -> Result {
    let terminal = ReplContext::init()?;
    commands.insert_resource(terminal);

    Ok(())
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

pub struct PromptPlugin{
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
        app.add_systems(Update, toggle_repl);
        app.add_systems(Update, bevy_keyboard_input_system.run_if(repl_is_enabled));
    }
}

#[derive(Resource)]
pub struct Repl {
    enabled: bool,
    prompt: String,
    toggle_key: Option<CrosstermKeyCode>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            enabled: true,
            prompt: "> ".to_string(),
            toggle_key: Some(CrosstermKeyCode::Char('r')),
        }
    }
}

fn repl_is_enabled(repl: Res<Repl>) -> bool {
    repl.enabled
}

fn toggle_repl(mut repl: ResMut<Repl>, mut key_events: EventReader<KeyEvent>) {
    if let Some(key) = repl.toggle_key {
        for event in key_events.read() {
            if event.code == key && event.kind == CrosstermKeyEventKind::Press {
                repl.enabled = !repl.enabled;
                info!("Repl is now {}", if repl.enabled { "enabled" } else { "disabled" });
            }
        }
    }
}

#[derive(Resource)]
struct ReplInput {
    buffer: String,
    cursor_pos: usize,
    prompt: String,
}

fn bevy_keyboard_input_system(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}
