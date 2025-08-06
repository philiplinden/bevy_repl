#![doc = include_str!("../README.md")]

pub mod terminal;   
pub mod commands;
#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;

pub mod prelude {
    pub use crate::{ReplPlugin, ReplResult, commands::{ReplCommand, ReplExt}};
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
}

use bevy::prelude::*;
use anyhow::Result;

/// The main REPL plugin
#[derive(Default)]
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Initialize REPL resources
        // app.init_resource::<Repl>();
    }
}

pub type ReplResult<T> = Result<T, anyhow::Error>;
