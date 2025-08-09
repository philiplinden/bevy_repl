//! Key event forwarding with REPL toggle example.
//!
//! Demonstrates:
//! - How Bevy keyboard events behave when the REPL is enabled/disabled
//! - Blocking Bevy key forwarding while REPL is enabled
//! - Reading `KeyboardInput` events and `ButtonInput<KeyCode>`
//! - Triggering Bevy behavior (Space -> ping) when REPL is disabled

use bevy::{app::ScheduleRunnerPlugin, prelude::*, input::keyboard::KeyboardInput};
use bevy_repl::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    println!("Pong");
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                // Run headless in the terminal
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            ReplPlugins.set(ReplPlugin::disabled()),
            EventDemoPlugin,
        ))
        .run();
}

struct EventDemoPlugin;

impl Plugin for EventDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_repl_command::<PingCommand>()
            .add_observer(on_ping)
            .add_systems(Update, (log_bevy_key_events, log_bevy_keyboard_input))
            .add_systems(Startup, instructions)
            .add_systems(Update, ping_from_bevy_key_events);
    }
}

fn log_bevy_key_events(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        println!("Keyboard input event: {:?} {:?}", event.state, event.key_code);
    }
}

fn log_bevy_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let keys = keyboard_input.get_pressed();
    if keys.len() > 0 {
        println!("Keys pressed: {:?}", keys.collect::<Vec<_>>());
    }
}

fn ping_from_bevy_key_events(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<PingCommand>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event.write(PingCommand);
    }
}

fn instructions() {
    println!();
    println!("Bevy REPL key events example!");
    println!();
    println!("Try typing a command:");
    println!("  `ping`    - Trigger the ping command (prints Pong)");
    println!("  `quit`    - Close the app");
    println!();
    println!("Tip: With the REPL disabled, press Space to trigger ping from Bevy.");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}
