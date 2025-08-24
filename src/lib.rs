#![doc = include_str!("../README.md")]

pub mod built_ins;
pub mod command;
pub mod plugin;
pub mod prompt;
pub mod repl;
pub mod print;
pub mod log_ecs;
pub mod context;

pub mod prelude {
    pub use crate::built_ins::ReplDefaultCommandsPlugin;
    #[cfg(not(feature = "derive"))]
    pub use crate::command::ReplCommand;
    pub use crate::command::{ReplAppExt, ReplResult};
    pub use crate::plugin::ReplPlugins;
    pub use crate::prompt::{
        PromptPlugin, ReplPrompt, ReplPromptConfig,
        renderer::{ActiveRenderer, PromptRenderPlugin, PromptRenderer, minimal::MinimalRenderer},
    };
    pub use crate::prompt::renderer::scroll::ScrollRegionReadySet;
    pub use crate::repl::{Repl, ReplBufferEvent, ReplPlugin, ReplSet, ReplSubmitEvent,
        repl_is_enabled,
    };
    pub use crate::context::FallbackTerminalContext;
    // Bring the robust printing macro into the prelude for convenient use.
    // This allows: `use bevy_repl::prelude::*;` then `repl_println!(...)`.
    pub use crate::repl_println;
    // Low-level printer if callers prefer a function over the macro.
    pub use crate::print::repl_print;

    pub use crate::log_ecs::{
        custom_layer as repl_log_custom_layer,
        tracing_to_repl_fmt,
        tracing_to_repl_fmt_with_level,
        LogEvent,
        print_log_events_system,
    };

    #[cfg(feature = "derive")]
    pub use bevy_repl_derive::ReplCommand;
}

#[cfg(feature = "stdout")]
pub use crate::plugin::StdoutRatatuiPlugins;