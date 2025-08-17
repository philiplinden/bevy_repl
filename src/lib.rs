#![doc = include_str!("../README.md")]

pub mod built_ins;
pub mod command;
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
    pub use crate::plugin::{MinimalReplPlugins, ReplPlugins};
    pub use crate::prompt::{
        PromptPlugin, config::{PromptHint, PromptSymbol, ReplPromptConfig},
        renderer::{ActiveRenderer, PromptRenderPlugin, PromptRenderer},
    };
    pub use crate::repl::{Repl, ReplBufferEvent, ReplPlugin, ReplSet, ReplSubmitEvent,
        repl_is_enabled,
    };
    // Low-level printer if callers prefer a function over the macro.
    pub use crate::print::repl_print;
    // Bring the robust printing macro into the prelude for convenient use.
    // This allows: `use bevy_repl::prelude::*;` then `repl_println!(...)`.
    pub use crate::repl_println;

    pub use crate::log_ecs::{
        LogEvent, custom_layer as repl_log_custom_layer, print_log_events_system,
        tracing_to_repl_fmt, tracing_to_repl_fmt_with_level,
    };

    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}
