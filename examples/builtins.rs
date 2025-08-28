//! Built-in commands and feature flags example for Bevy REPL.
//!
//! Demonstrates:
//! - How built-in REPL commands are controlled via Cargo features
//! - Default set: `default_commands` enables `quit`, `clear`, and `help`
//! - Enabling/disabling individual commands with `--features` flags
//!
//! Run examples (from the crate root):
//! - With defaults (quit, clear, help):
//!   cargo run --example builtins
//! - Disable all defaults, enable only help:
//!   cargo run --example builtins --no-default-features --features "help"
//! - Disable all defaults, enable only quit:
//!   cargo run --example builtins --no-default-features --features "quit"
//! - Enable a custom subset:
//!   cargo run --example builtins --no-default-features --features "quit,clear"
//!
//! In the REPL, try typing:
//!   help
//!   clear
//!   quit

use bevy::prelude::*;
use bevy_repl::prelude::*;

fn yes_no(v: bool) -> &'static str {
    if v { "ENABLED" } else { "disabled" }
}

fn instructions() {
    repl_println!();
    repl_println!("Bevy REPL built-in commands (feature flags) example");
    repl_println!();
    repl_println!("This example shows how built-in commands are compiled via features.");
    repl_println!("Available built-ins at compile-time:");

    repl_println!();
    repl_println!("Try these commands in the REPL:");
    repl_println!("  quit   - Exit the app ({})", yes_no(cfg!(feature = "quit")));
    repl_println!("  help   - Show available commands ({})", yes_no(cfg!(feature = "help")));
    repl_println!("  clear  - Clear the screen ({})", yes_no(cfg!(feature = "clear")));
    repl_println!();
    repl_println!("To control which are enabled, run with Cargo feature flags.");
    repl_println!("Examples:");
    repl_println!("  cargo run --example builtins");
    repl_println!("  cargo run --example builtins --no-default-features --features \"help\"");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            )),
            ReplPlugins,
        ))
        .add_systems(PostStartup, instructions)
        .run();
}
