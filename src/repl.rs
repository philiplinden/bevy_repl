use crate::{command::CommandParser, keymap::ReplKeymap};

use bevy::prelude::*;
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
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone, Copy, Debug, Default)]
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Arm panic and signal safety hooks synchronously on plugin build
        install_safety_hooks();

        if !app.is_plugin_added::<bevy::log::LogPlugin>() {
            let _ = tracing_subscriber::registry()
                .with(crate::tracing::make_repl_layer())
                .try_init();
        }
        app.init_resource::<Repl>();
        app.add_message::<ReplBufferEvent>();
        app.add_systems(Startup, init_repl);
        app.add_systems(Last, on_app_exit_restore);
        app.configure_sets(
            PreUpdate,
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
        app.configure_sets(PreUpdate, ReplSet::Print.run_if(repl_is_enabled));
    }
}

fn install_safety_hooks() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = Repl::restore_terminal();
        default_hook(panic_info);
    }));

    let _ = ctrlc::set_handler(move || {
        let _ = Repl::restore_terminal();
        std::process::exit(0);
    });
}

fn init_repl(repl: Res<Repl>) {
    let _ = crossterm::terminal::enable_raw_mode();
    if repl.enabled {
        if let Ok((_, rows)) = crossterm::terminal::size() {
            crate::print::set_scroll_region(rows.saturating_sub(1));
        }
    }
}

fn on_app_exit_restore(mut exit: MessageReader<AppExit>) {
    for _ in exit.read() {
        let _ = Repl::restore_terminal();
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
            enabled: true,
            prompt_symbol: "> ".to_string(),
            buffer: String::new(),
            cursor_pos: 0,
            keymap: ReplKeymap::default(),
            commands: HashMap::new(),
        }
    }
}

impl Repl {
    /// Create a REPL resource configuration that starts enabled (default).
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Self::default()
        }
    }

    /// Create a REPL resource configuration that starts disabled.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    pub fn enable(&mut self) {
        if !self.enabled {
            self.enabled = true;
            if let Ok((_, rows)) = crossterm::terminal::size() {
                crate::print::set_scroll_region(rows.saturating_sub(1));
            }
        }
    }

    pub fn disable(&mut self) {
        if self.enabled {
            self.enabled = false;
            self.clear_buffer();
            let (_, rows) = crossterm::terminal::size().unwrap_or((80, 24));
            crate::print::clear_prompt_line(rows.saturating_sub(1));
            crate::print::reset_scroll_region();
        }
    }
    pub fn toggle(&mut self) {
        if self.enabled {
            self.disable();
        } else {
            self.enable();
        }
    }

    pub fn init_terminal() -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        if let Ok((_, rows)) = crossterm::terminal::size() {
            crate::print::set_scroll_region(rows.saturating_sub(1));
        }
        Ok(())
    }

    pub fn restore_terminal() -> std::io::Result<()> {
        let (_, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let prompt_row = rows.saturating_sub(1);

        crate::print::clear_prompt_line(prompt_row);
        crate::print::reset_scroll_region();

        let mut out = std::io::stdout();
        let _ = crossterm::queue!(
            out,
            crossterm::cursor::MoveTo(0, prompt_row),
            crossterm::cursor::Show
        );
        let _ = std::io::Write::write(&mut out, b"\r\n");
        let _ = std::io::Write::flush(&mut out);

        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_terminal() -> std::io::Result<()> {
        use crossterm::{
            cursor::MoveTo,
            execute,
            terminal::{Clear, ClearType},
        };
        let mut out = std::io::stdout();
        execute!(
            out,
            Clear(ClearType::All),
            Clear(ClearType::Purge),
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
