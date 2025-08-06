#![doc = include_str!("../README.md")]

pub mod repl;
pub mod commands;
#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;

pub mod prelude {
    pub use crate::repl::{PromptPlugin, Repl, ReplPlugins};
    pub use crate::{commands::{ReplCommand, ReplExt, ReplResult}};
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
}
