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
    pub use crate::plugin::{MinimalReplPlugins, ReplPlugins};
    pub use crate::prompt::{
        PromptPlugin, ReplPrompt, ReplPromptConfig,
        renderer::{ActiveRenderer, PromptRenderPlugin, PromptRenderer, minimal::MinimalRenderer},
    };
    pub use crate::repl::{
        FallbackTerminalContext, Repl, ReplBufferEvent, ReplPlugin, ReplSet, ReplSubmitEvent,
        repl_is_enabled,
    };
    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}
