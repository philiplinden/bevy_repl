//! REPL can modify resources at runtime example.
//!
//! Demonstrates:
//! - Defining a resource (`TimeScale`)
//! - REPL command that reads/mutates a resource via an observer
//! - Other systems reading the updated resource live
//!
//! Try:
//!   time-scale            # prints current value
//!   time-scale --set 2.0  # sets absolute value
//!   time-scale --add -0.5 # adds delta to current value

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

// -------- Resource we will mutate at runtime --------
#[derive(Resource, Debug, Clone, Copy)]
struct TimeScale(pub f32);

// A tiny frame counter to periodically show the current value without spamming
#[derive(Resource, Debug, Default)]
struct FrameCounter(pub u32);

// -------- REPL command definition --------
#[derive(Debug, Clone, Event, Default)]
struct TimeScaleCommand {
    set: Option<f32>,
    add: Option<f32>,
}

impl ReplCommand for TimeScaleCommand {
    fn clap_command() -> clap::Command {
        use clap::{value_parser, Arg, ArgGroup, Command};

        Command::new("time-scale")
            .about("Get or modify the TimeScale resource")
            .arg(
                Arg::new("set")
                    .long("set")
                    .num_args(1)
                    .value_parser(value_parser!(f32))
                    .help("Set absolute time scale (>= 0.0)"),
            )
            .arg(
                Arg::new("add")
                    .long("add")
                    .num_args(1)
                    .value_parser(value_parser!(f32))
                    .help("Add delta to current time scale"),
            )
            .group(
                ArgGroup::new("mode")
                    .args(["set", "add"]) // at most one option
                    .multiple(false)
                    .required(false),
            )
            .after_help("If no flags are provided, prints the current TimeScale.")
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let set = matches.get_one::<f32>("set").copied();
        let add = matches.get_one::<f32>("add").copied();
        Ok(TimeScaleCommand { set, add })
    }
}

// -------- Observer that mutates the resource --------
fn on_time_scale(trigger: Trigger<TimeScaleCommand>, mut scale: ResMut<TimeScale>) {
    let cmd = trigger.event();

    match (cmd.set, cmd.add) {
        (Some(value), None) => {
            let new = value.max(0.0);
            scale.0 = new;
            repl_println!("TimeScale set to: {}", scale.0);
        }
        (None, Some(delta)) => {
            let new = (scale.0 + delta).max(0.0);
            scale.0 = new;
            repl_println!("TimeScale changed by {delta:+}, now: {}", scale.0);
        }
        (None, None) => {
            repl_println!("Current TimeScale: {}", scale.0);
        }
        (Some(_), Some(_)) => {
            // Should be prevented by ArgGroup, but handle defensively
            repl_println!("Error: specify at most one of --set or --add");
        }
    }
}

// -------- Systems that read the resource live --------
fn tick(mut frames: ResMut<FrameCounter>, scale: Res<TimeScale>) {
    frames.0 = frames.0.wrapping_add(1);
    if frames.0 % 60 == 0 {
        repl_println!("Tick: frame={}, time-scale={}", frames.0, scale.0);
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Bevy REPL resource mutation example");
    repl_println!();
    repl_println!("Try typing in the REPL:");
    repl_println!("  time-scale");
    repl_println!("  time-scale --set 2.0");
    repl_println!("  time-scale --add -0.5");
    repl_println!("  quit");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(TimeScale(1.0));
    commands.insert_resource(FrameCounter::default());
}

fn main() {
    App::new()
        .add_plugins((
            // Headless loop in the terminal
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            // Required so the REPL can handle keyboard input
            bevy::input::InputPlugin::default(),
            // Turnkey REPL with minimal prompt renderer and default commands
            ReplPlugins,
        ))
        .add_repl_command::<TimeScaleCommand>()
        .add_observer(on_time_scale)
        .add_systems(Startup, setup)
        .add_systems(Update, tick)
        .add_systems(PostStartup, instructions.after(ScrollRegionReadySet))
        .run();
}
