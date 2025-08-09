//! Key event forwarding with REPL toggle example.
//!
//! Demonstrates:
//! - How Bevy keyboard events behave when the REPL is enabled/disabled
//! - Blocking Bevy key forwarding while REPL is enabled
//! - Reading `KeyboardInput` events and `ButtonInput<KeyCode>`
//! - Triggering Bevy behavior (Space -> ping) when REPL is disabled

use bevy::{
    app::ScheduleRunnerPlugin,
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
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
            .add_systems(
                Update,
                (
                    log_bevy_key_events_before_repl,
                    log_bevy_button_input_before_repl,
                )
                    .before(ReplSet::Pre),
            )
            .add_systems(
                Update,
                (
                    log_bevy_key_events_after_repl,
                    log_bevy_button_input_after_repl,
                    ping_from_bevy_input,
                )
                    .after(ReplSet::Post),
            )
            .add_systems(Startup, instructions);
    }
}

fn log_bevy_key_events_before_repl(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        println!(
            "(Before REPL) Keyboard input event: {:?} {:?}",
            event.state, event.key_code
        );
    }
}

fn log_bevy_button_input_before_repl(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_just_pressed().len() > 0 {
        println!("(Before REPL) Button Input: {:?}", keyboard_input);
    }
}

fn log_bevy_key_events_after_repl(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        println!(
            "(After REPL) Keyboard input event: {:?} {:?}",
            event.state, event.key_code
        );
    }
}

fn log_bevy_button_input_after_repl(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_just_pressed().len() > 0 {
        println!("(After REPL) Button Input: {:?}", keyboard_input);
    }
}

fn ping_from_bevy_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut key_events: EventReader<KeyboardInput>,
    mut event: EventWriter<PingCommand>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("\nPing from Bevy Button Input resource, not the REPL\n");
        event.write(PingCommand);
    }
    for event in key_events.read() {
        if event.key_code == KeyCode::Space && event.state == ButtonState::Pressed {
            println!("\nPing from Bevy KeyboardInput Event, not the REPL\n");
        }
    }
}

fn instructions() {
    println!("The REPL is disabled, so key events are forwarded to Bevy.");
    println!();
    println!("Press Space to trigger ping from Bevy.");
    println!();
    println!("Press CTRL+C to exit any time.");
    println!();
}
