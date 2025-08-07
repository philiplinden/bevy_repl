#![doc = include_str!("../README.md")]

#[cfg(any(feature = "default_commands", feature = "quit"))]
pub mod built_ins;
pub mod command;
pub mod plugin;
pub mod prompt;
pub mod repl;

pub mod prelude {
    #[cfg(any(feature = "default_commands", feature = "quit"))]
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    pub use crate::command::{ParserPlugin, ReplCommand, ReplResult, ReplAppExt, CommandParser};
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::{PromptPlugin, ReplPrompt, ReplSubmitEvent, ReplBufferEvent};
    pub use crate::repl::{ReplPlugin, Repl, repl_is_enabled, ReplToggleEvent, ReplContext};
}
