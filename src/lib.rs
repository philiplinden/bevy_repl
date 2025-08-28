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
pub mod context;
pub mod log_ecs;
pub mod plugin;
pub mod print;
pub mod prompt;
pub mod repl;

pub mod prelude {
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    #[cfg(not(feature = "derive"))]
    pub use crate::command::ReplCommand;
    pub use crate::command::{ReplAppExt, ReplResult};
    pub use crate::prompt::{
        PromptPlugin, ReplPrompt, ReplPromptConfig,
        renderer::{ActiveRenderer, PromptRenderPlugin, PromptRenderer, simple::SimpleRenderer},
        keymap::{Binding as ReplKeybind, PromptKeymap},
    };
    pub use crate::repl::{
        Repl, ReplBufferEvent, ReplPlugin, ReplSet, ReplSubmitEvent, repl_is_enabled,
    };
    // Bring the robust printing macro into the prelude for convenient use.
    // This allows: `use bevy_repl::prelude::*;` then `repl_println!(...)`.
    pub use crate::repl_println;
    // Low-level printer if callers prefer a function over the macro.
    pub use crate::print::repl_print;

    pub use crate::context::ReplContextPlugin;
    pub use crate::log_ecs::{
        LogEvent, custom_layer as repl_log_custom_layer, print_log_events_system,
        tracing_to_repl_fmt, tracing_to_repl_fmt_with_level,
    };
    pub use crate::plugin::{ReplPlugins, StdoutRatatuiPlugin};

    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;

    // re-exports for convenience
    pub use bevy_ratatui::crossterm::event::{KeyCode as CrosstermKey, KeyModifiers as CrosstermMods};
}

// re-export at the root for convenience
pub use crate::prelude::ReplPlugins;
