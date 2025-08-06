#![doc = include_str!("../README.md")]

#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;
pub mod commands;
pub mod plugin;
pub mod prompt;
pub mod repl;

pub mod prelude {
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    pub use crate::commands::{ReplCommand, ReplExt, ReplResult};
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::PromptPlugin;
    pub use crate::repl::ReplPlugin;
}
