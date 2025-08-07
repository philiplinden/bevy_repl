#![doc = include_str!("../README.md")]

#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;
pub mod parse;
pub mod plugin;
pub mod prompt;
pub mod repl;

pub mod prelude {
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    pub use crate::parse::{ReplCommand, ReplResult, ReplAppExt, CommandParser};
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::PromptPlugin;
    pub use crate::repl::ReplPlugin;
}
