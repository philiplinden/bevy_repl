#![doc = include_str!("../README.md")]

pub mod built_ins;
pub mod terminal;   
pub mod commands;

pub mod prelude {
    pub use crate::{Repl, ReplConfig, ReplPlugin, ReplResult, ReplSet, ReplCommand, ReplCommandExt};
}

use anyhow::{Context, Result, anyhow, bail, ensure};

use crate::terminal::BevyCrosstermTerminal;
use bevy::prelude::*;
use bevy_crossterm::prelude::*;

/// Trait for commands that can be registered with the REPL
/// Commands should derive clap::Parser and implement this trait
pub trait ReplCommand: clap::Parser + Send + Sync + 'static {
    /// The observer function type for this command
    type Observer: Fn(Trigger<Self>) + Send + Sync + 'static;
}

/// Extension trait for App to register REPL commands
pub trait ReplCommandExt {
    /// Register a command with its observer function
    fn repl<C: ReplCommand>(&mut self, observer: C::Observer) -> &mut Self;
}

impl ReplCommandExt for App {
    fn repl<C: ReplCommand>(&mut self, observer: C::Observer) -> &mut Self {
        // TODO: Implement command registration using Bevy's observer system
        // This will parse commands and trigger the observer when matched
        self.add_observer(observer)
    }
}

/// The main REPL plugin
#[derive(Default)]
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Add bevy_crossterm plugin first
        app.add_plugins(CrosstermPlugin);

        // Insert the configuration resource
        let config = self.config.clone();
        let mut repl = Repl::with_config(config.clone());
        if config.enabled_on_startup {
            repl.enable();
        }
        app.insert_resource(config)
            .insert_resource(repl)
            .configure_sets(Update, ReplSet::First); // Always runs

        // Add cleanup system
        app.add_systems(Update, exit_on_interrupt);
    }
}

/// This handles the crossterm interrupt event and exits the app.
///
/// Without this, the terminal might hang with no way to exit the app because
/// the exit event is not handled.
fn exit_on_interrupt(
    mut interrupt_events: EventReader<CrosstermInputEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for event in interrupt_events.read() {
        if let CrosstermInputEvent::Interrupt = event {
            exit.send(AppExit::Success);
        }
    }
}

// Use anyhow::Error directly - this is the standard pattern
pub type ReplResult<T> = Result<T, anyhow::Error>;

/// The SystemSet for console/command related systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReplSet {
    /// Systems that run before anything else, like toggling the REPL
    First,

    /// Systems operating the console UI (the input layer)
    Parse,

    /// Systems executing console commands (the functionality layer).
    /// All command handler systems are added to this set
    Trigger,
}

pub struct ReplTerminal {
    terminal: Option<BevyCrosstermTerminal>,
}

impl Default for ReplTerminal {
    fn default() -> Self {}
}

impl ReplTerminal {
    fn new() -> Self {
        Self {
            terminal: None,
        }
    }

    fn spawn(&mut self) {
        if self.terminal.is_some() {
            return;
        }

        let mut terminal = BevyCrosstermTerminal::new(
            "> ".to_string(),
            None,
        );

        if let Err(e) = terminal.init() {
            eprintln!("Failed to initialize terminal: {}", e);
            return;
        }

        self.terminal = Some(terminal);
    }

    fn shutdown(&mut self) {
        if let Some(mut terminal) = self.terminal.take() {
            terminal.cleanup().ok();
        }
    }

    fn try_recv_input(&mut self) -> Option<String> {
        if let Some(terminal) = &mut self.terminal {
            terminal.poll_input()
        } else {
            None
        }
    }

    fn send_output(&mut self, output: String) {
        if let Some(terminal) = &mut self.terminal {
            terminal.print_output(&output);
        }
    }
}
