//! Aliases example for Bevy REPL using clap.
//!
//! Demonstrates:
//! - Defining a REPL command with multiple aliases via clap
//! - All aliases map to the same command implementation transparently
//!
//! Try typing in the REPL (all do the same thing):
//!   remove target_file
//!   rm target_file
//!   del target_file
//!   quit

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Event, Default)]
struct RemoveCommand {
    target: String,
}

impl ReplCommand for RemoveCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("remove")
            .about("Remove a target (demo only)")
            // Add aliases recognized by clap
            .visible_alias("rm")
            .visible_alias("del")
            .arg(
                clap::Arg::new("target")
                    .help("Target to remove (demo)")
                    .required(true),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let target = match matches.get_one::<String>("target").cloned() {
            Some(t) => t,
            None => {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "missing required argument: <target>",
                ))
            }
        };
        Ok(Self { target })
    }
}

fn on_remove(trigger: Trigger<RemoveCommand>) {
    let ev = trigger.event();
    println!("Pretending to remove: {}", ev.target);
}

fn instructions() {
    println!();
    println!("Bevy REPL aliases example (clap)");
    println!();
    println!("These are all equivalent:");
    println!("  remove <target>");
    println!("  rm <target>");
    println!("  del <target>");
    println!();
    println!("The REPL can be toggled with:");
    println!("  {:?}", Repl::default().toggle_key.unwrap());
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            ReplPlugins,
        ))
        .add_repl_command::<RemoveCommand>()
        .add_observer(on_remove)
        .add_systems(Startup, instructions)
        .run();
}
