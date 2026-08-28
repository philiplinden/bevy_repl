//! A Bevy plugin that provides a Read-Eval-Print Loop (REPL) interface for
//! interactive command input.
//!
//! # Purpose
//! The `ReplPlugins` group enables a REPL within the terminal while your Bevy
//! application runs, allowing users to enter commands and interact with the
//! Bevy ECS at runtime.
//!
//! # Usage
//! Add the plugin to your Bevy app:
//! ```rust
//! use bevy_repl::ReplPlugins;
//! App::new().add_plugins(ReplPlugins);
//! ```

#![doc = include_str!("../README.md")]

pub mod built_ins;
pub mod command;
pub mod input;
pub mod keymap;
pub mod print;
pub mod repl;
pub mod tracing;

pub mod prelude {
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    #[cfg(not(feature = "derive"))]
    pub use crate::command::ReplCommand;
    pub use crate::command::{ReplAppExt, ReplResult};
    pub use crate::input::InputPlugin;
    pub use crate::keymap::{Binding as ReplKeybind, ReplKeymap};
    pub use crate::repl::{
        Repl, ReplBufferEvent, ReplPlugin, ReplSet, ReplSubmitEvent, repl_is_enabled,
    };
    // Bring the robust printing macro into the prelude for convenient use.
    // This allows: `use bevy_repl::prelude::*;` then `repl_println!(...)`.
    pub use crate::repl_println;
    // Low-level printer if callers prefer a function over the macro.
    pub use crate::print::repl_print;

    pub use crate::tracing::{
        LogEvent, custom_layer as repl_log_custom_layer, print_log_events_system,
        tracing_to_repl_fmt, tracing_to_repl_fmt_with_level,
    };

    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Default REPL plugin group: includes ratatui plugins and default commands.
///
/// This is the turnkey setup most users want. It wires in the REPL core, prompt
/// plugin, parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::input::InputPlugin)
            .add(crate::keymap::ReplKeymapPlugin)
            .add(crate::tracing::ReplLogPrintPlugin)
            // only adds commands that are enabled by feature flags
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}
