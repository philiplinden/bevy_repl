//! Resource interaction example for Bevy REPL.
//!
//! Demonstrates:
//! - Creating and mutating a Bevy `Resource`
//! - Defining multiple REPL commands (start/stop/reset)
//! - Emitting ECS events from parsed commands
//! - Running conditional systems with `run_if`
//! - FixedUpdate for time-based behavior
//! - Running headless via `ScheduleRunnerPlugin`
//! - Toggling the REPL and entering commands in the terminal
use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;

// Timer state enum
#[derive(Debug, Clone, PartialEq, Eq)]
enum TimerState {
    Stopped,
    Running,
}

// Timer resource that holds both the timer and its state
#[derive(Resource, Debug)]
struct TimerResource {
    timer: Timer,
    state: TimerState,
    elapsed_seconds: f32,
}

impl Default for TimerResource {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            state: TimerState::Stopped,
            elapsed_seconds: 0.0,
        }
    }
}

impl TimerResource {
    fn start(&mut self) {
        self.state = TimerState::Running;
        self.timer.reset();
        info!("Timer started");
    }

    fn stop(&mut self) {
        self.state = TimerState::Stopped;
        info!("Timer stopped at {:.2} seconds", self.elapsed_seconds);
    }

    fn reset(&mut self) {
        self.elapsed_seconds = 0.0;
        self.timer.reset();
        self.state = TimerState::Stopped;
        info!("Timer reset to 0.00 seconds");
    }

    fn is_running(&self) -> bool {
        self.state == TimerState::Running
    }
}

// Command structs
#[derive(Debug, Clone, Event, Default)]
struct StartTimerCommand;

impl ReplCommand for StartTimerCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("start")
            .about("Starts the timer")
    }
}

#[derive(Debug, Clone, Event, Default)]
struct StopTimerCommand;

impl ReplCommand for StopTimerCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("stop")
            .about("Stops the timer")
    }
}

#[derive(Debug, Clone, Event, Default)]
struct ResetTimerCommand;

impl ReplCommand for ResetTimerCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("reset")
            .about("Resets the timer to 0")
    }
}

// Command handlers
fn on_start_timer(_trigger: Trigger<StartTimerCommand>, mut timer: ResMut<TimerResource>) {
    timer.start();
}

fn on_stop_timer(_trigger: Trigger<StopTimerCommand>, mut timer: ResMut<TimerResource>) {
    timer.stop();
}

fn on_reset_timer(_trigger: Trigger<ResetTimerCommand>, mut timer: ResMut<TimerResource>) {
    timer.reset();
}

fn timer_is_running(timer: Res<TimerResource>) -> bool {
    timer.is_running()
}

fn display_timer_status(timer: Res<TimerResource>) {
    let state_str = match timer.state {
        TimerState::Running => "Running",
        TimerState::Stopped => "Stopped",
    };
    info!("Timer Status: {} | Elapsed: {:.2} seconds", state_str, timer.elapsed_seconds);
}

// System that increments the timer when running
fn update_timer(mut timer: ResMut<TimerResource>, time: Res<Time>) {
    if timer.is_running() {
        timer.timer.tick(time.delta());
        
        // Increment elapsed time by the delta time
        timer.elapsed_seconds += time.delta_secs();
        
        // Log every second when running
        if timer.timer.just_finished() {
            info!("Timer: {:.2} seconds", timer.elapsed_seconds);
        }
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL resource example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `start`   - Start the timer");
    repl_println!("  `stop`    - Stop the timer");
    repl_println!("  `reset`   - Reset the timer to 0");
    repl_println!("  `quit`    - Close the app");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}


fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            bevy::input::InputPlugin::default(),
            bevy::log::LogPlugin::default(),
            bevy_ratatui::RatatuiPlugins::default(),
            ReplPlugins,
        ))
        .init_resource::<TimerResource>()
        .add_repl_command::<StartTimerCommand>()
        .add_repl_command::<StopTimerCommand>()
        .add_repl_command::<ResetTimerCommand>()
        .add_observer(on_start_timer)
        .add_observer(on_stop_timer)
        .add_observer(on_reset_timer)
        .add_systems(Update, display_timer_status.run_if(timer_is_running))
        .add_systems(FixedUpdate, update_timer)
        .add_systems(PostStartup, instructions.after(ScrollRegionReadySet))
        .run();
}
