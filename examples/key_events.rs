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
            ReplPlugins,
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
            (log_bevy_key_events, log_bevy_keyboard_input),
        )
        .add_systems(Startup, instructions)
        .add_systems(
            Update,
            ping_from_bevy_key_events,
        )
        .add_observer(info_on_toggle);
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

fn ping_from_bevy_key_events(keyboard_input: Res<ButtonInput<KeyCode>>, mut event: EventWriter<PingCommand>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event.write(PingCommand);
    }
}

fn instructions() {
    println!("This demo shows how bevy_repl uses crossterm events for input and 
    blocks key events from being forwarded to Bevy when the REPL is enabled.");
    println!("With the REPL enabled, type `ping` then hit Enter to trigger the ping command.");
    println!("With the REPL disabled, hit space to trigger the ping command.");
    println!("Press {:?} to toggle the REPL.", Repl::default().toggle_key.unwrap());
    println!("Close the app with the `quit` command from the REPL. Press CTRL+C to exit any time.");
}

fn info_on_toggle(trigger: Trigger<ReplToggleEvent>) {
    match trigger.event() {
        ReplToggleEvent::Enable => println!("Key events are blocked from Bevy. Type `ping` then hit Enter to trigger the ping command."),
        ReplToggleEvent::Disable => println!("Key events are forwarded to Bevy. Hit space to trigger the ping command."),
    }
}