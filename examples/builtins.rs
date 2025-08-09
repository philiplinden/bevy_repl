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

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            // Headless loop in the terminal
            MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            ReplPlugins,
        ))
        .add_systems(Startup, instructions)
        .run();
}

fn instructions() {
    println!();
    println!("Bevy REPL built-in commands (feature flags) example");
    println!();
    println!("This example shows how built-in commands are compiled via features.");
    println!("Available built-ins at compile-time:");

    // Report which built-ins were compiled in
    println!("  help : {}", yes_no(cfg!(feature = "help")));
    println!("  clear: {}", yes_no(cfg!(feature = "clear")));
    println!("  quit : {}", yes_no(cfg!(feature = "quit")));

    println!();
    println!("Try these commands in the REPL:");
    println!("  help   - Show available commands (if enabled)");
    println!("  clear  - Clear the REPL screen (if enabled)");
    println!("  quit   - Exit the app (if enabled)");
    println!();
    println!("To control which are enabled, run with Cargo feature flags.");
    println!("Examples:");
    println!("  cargo run --example builtins");
    println!("  cargo run --example builtins --no-default-features --features \"help\"");
    println!("  cargo run --example builtins --no-default-features --features \"quit,clear\"");
    println!();
    println!("The REPL can be toggled with:");
    println!("  {:?}", Repl::default().toggle_key.unwrap());
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn yes_no(v: bool) -> &'static str {
    if v { "ENABLED" } else { "disabled" }
}
