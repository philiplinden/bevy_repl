#![doc = include_str!("../README.md")]

pub mod built_ins;
pub mod command;
pub mod plugin;
pub mod prompt;
pub mod repl;

pub mod prelude {
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    #[cfg(not(feature = "derive"))]
    pub use crate::command::ReplCommand;
    pub use crate::command::{ReplAppExt, ReplResult};
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::{PromptPlugin, ReplPrompt};
    pub use crate::repl::{
        Repl, ReplBufferEvent, ReplContext, ReplPlugin, ReplSubmitEvent,
        repl_is_enabled,
    };
    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}
