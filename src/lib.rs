#![doc = include_str!("../README.md")]

pub mod plugin;
pub mod commands;
pub mod prompt;
#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;

pub mod prelude {
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::{PromptPlugin, Repl};
    pub use crate::{commands::{ReplCommand, ReplExt, ReplResult}};
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
}
