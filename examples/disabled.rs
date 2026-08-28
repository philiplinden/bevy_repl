//! Example showing how to start an app with the REPL disabled by default.
//!
//! Demonstrates:
//! - Configuring `ReplPlugins.set(ReplPlugin::disabled())`
//! - Starting up in normal console mode with no prompt or raw-mode overhead
//! - Pressing the toggle key (` ` `) in Bevy input to open the REPL at runtime
//! - Pressing ` ` ` again to close the REPL and return to normal console mode

use bevy::log::info;
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }

    fn to_event(_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self)
    }
}

fn on_ping(_trigger: On<PingCommand>) {
    info!("Pong! REPL is active.");
}

/// A system that listens for the toggle key when the REPL is disabled.
fn toggle_when_disabled(
    keyboard: Res<ButtonInput<KeyCode>>,
    repl: Res<Repl>,
    mut lifecycle_events: MessageWriter<ReplLifecycleEvent>,
) {
    if !repl.enabled && keyboard.just_pressed(KeyCode::Backquote) {
        info!("Opening REPL...");
        lifecycle_events.write(ReplLifecycleEvent::Enable);
    }
}

/// Simulated game system running in the background.
fn background_game_loop(time: Res<Time>, mut timer: Local<Option<Timer>>) {
    let t = timer.get_or_insert_with(|| Timer::from_seconds(2.0, TimerMode::Repeating));
    if t.tick(time.delta()).just_finished() {
        info!("Game tick: elapsed = {:.1}s", time.elapsed_secs());
    }
}

fn instructions() {
    println!();
    println!("=== App Started with REPL Disabled ===");
    println!("The app is currently running in standard console mode.");
    println!("Press '`' (backtick) at any time to OPEN the interactive REPL.");
    println!("Press '`' again while open to CLOSE the REPL.");
    println!("Press CTRL+C to exit.");
    println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_secs_f64(1.0 / 60.0),
                ))
                .adapt_for_repl(),
            ReplPlugins,
        ))
        // Start with the REPL disabled:
        .insert_resource(Repl::disabled())
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(Startup, instructions)
        .add_systems(Update, (toggle_when_disabled, background_game_loop))
        .run();
}
