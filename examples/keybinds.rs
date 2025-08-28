//! Custom keybinds example for Bevy REPL.
//!
//! Demonstrates how to configure the PromptKeymap resource to use
//! exact key + modifier combinations for REPL actions, and how the
//! fallback character insertion works.
//!
//! Note: The REPL uses Crossterm keycodes and modifiers to capture input,
//! NOT Bevy keycodes and modifiers.

use bevy::prelude::*;
use bevy_ratatui::crossterm::event::{KeyCode as CrosstermKeyCode, KeyModifiers};
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Test command")
    }
}

fn on_ping(_trigger: Trigger<PingCommand>) {
    repl_println!("Pong");
}

struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_repl_command::<PingCommand>().add_observer(on_ping);
        app.add_systems(
            Update,
            (
                use_custom_keybinds,
                use_default_keybinds,
                clear_all_keybinds,
            )
                // Run before the REPL system so the key events don't compete
                // with the REPL
                .in_set(ReplSet::Pre),
        );
    }
}

fn use_custom_keybinds(mut commands: Commands, bevy_input: Res<ButtonInput<KeyCode>>) {
    if bevy_input.any_just_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && bevy_input.just_pressed(KeyCode::KeyA)
    {
        commands.insert_resource(PromptKeymap {
            submit: Some(ReplKeybind {
                code: CrosstermKeyCode::Enter,
                mods: KeyModifiers::SHIFT,
            }),
            delete: Some(ReplKeybind {
                code: CrosstermKeyCode::Delete,
                mods: KeyModifiers::NONE,
            }),
            clear: Some(ReplKeybind {
                code: CrosstermKeyCode::Char('D'),
                mods: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            }),
            ..default()
        });
    }
}

fn use_default_keybinds(mut commands: Commands, bevy_input: Res<ButtonInput<KeyCode>>) {
    if bevy_input.any_just_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && bevy_input.just_pressed(KeyCode::Backquote)
    {
        commands.insert_resource(PromptKeymap::default());
    }
}

fn clear_all_keybinds(mut commands: Commands, bevy_input: Res<ButtonInput<KeyCode>>) {
    if bevy_input.just_pressed(KeyCode::Backquote) {
        commands.insert_resource(PromptKeymap::none());
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Bevy REPL custom keybinds example");
    repl_println!();
    repl_println!("This example shows how Bevy key events and REPL keybinds can");
    repl_println!("be configured to customize the REPL, even at runtime.");
    repl_println!();
    repl_println!("Controls (always active)");
    repl_println!("------------------------");
    repl_println!("Switch to custom keybinds:   Ctrl+A");
    repl_println!("Reset keybinds to defaults:  Ctrl+`");
    repl_println!("Clear all keybinds:               `");
    repl_println!("Exit the app:                Ctrl+C");
    repl_println!();
    repl_println!("Default Keybinds");
    repl_println!("----------------");
    repl_println!("Submit buffer:                Enter");
    repl_println!("Clear buffer:                   Esc");
    repl_println!();
    repl_println!("Custom Keybinds");
    repl_println!("----------------");
    repl_println!("Submit buffer:           Shift+Enter");
    repl_println!("Clear buffer:           Ctrl+Shift+D");
    repl_println!();
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )),
        ReplPlugins,
        ExamplePlugin,
    ))
    .add_systems(PostStartup, instructions)
    .run();
}
