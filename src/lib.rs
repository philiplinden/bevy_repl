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
    pub use crate::command::ReplCommand;
    pub use crate::command::{ReplAppExt, ReplResult};
    pub use crate::input::InputPlugin;
    pub use crate::keymap::{Binding as ReplKeybind, ReplKeymap};
    pub use crate::repl::{
        Repl, ReplBufferEvent, ReplLifecycleEvent, ReplPlugin, ReplSet, ReplSubmitEvent,
        repl_is_enabled,
    };
    // Bring the robust printing macro into the prelude for convenient use.
    pub use crate::repl_println;
    // Low-level printer if callers prefer a function over the macro.
    pub use crate::print::repl_print;

    pub use crate::ReplDefaultPluginsExt;
    pub use crate::ReplPlugins;
    pub use crate::tracing::repl_tracing_layer;

    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::DefaultPlugins;

/// Default REPL plugin group.
///
/// This is the turnkey setup most users want. It wires in the REPL core, input capture,
/// keymap, command parser, and the default commands.
pub struct ReplPlugins;

impl PluginGroup for ReplPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::repl::ReplPlugin::default())
            .add(crate::command::ParserPlugin)
            .add(crate::input::InputPlugin)
            .add(crate::keymap::InputKeymapPlugin)
            .add(crate::print::PrintPlugin)
            .add(crate::built_ins::ReplDefaultCommandsPlugin)
    }
}

/// Extension trait for configuring `DefaultPlugins` with scroll-safe REPL log routing.
pub trait ReplDefaultPluginsExt {
    /// Configures Bevy's `LogPlugin` to route tracing logs into the REPL's scroll region.
    fn adapt_for_repl(self) -> PluginGroupBuilder;
}

impl ReplDefaultPluginsExt for DefaultPlugins {
    fn adapt_for_repl(self) -> PluginGroupBuilder {
        self.set(bevy::log::LogPlugin {
            custom_layer: crate::tracing::repl_tracing_layer,
            ..Default::default()
        })
    }
}

impl ReplDefaultPluginsExt for PluginGroupBuilder {
    fn adapt_for_repl(self) -> PluginGroupBuilder {
        self.set(bevy::log::LogPlugin {
            custom_layer: crate::tracing::repl_tracing_layer,
            ..Default::default()
        })
    }
}
