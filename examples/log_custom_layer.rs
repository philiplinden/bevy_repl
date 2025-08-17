use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

// Demonstrates wiring Bevy's LogPlugin.custom_layer to route tracing logs
// into bevy_repl via a capture plumbing plugin and a custom layer builder.
//
// Notes:
// - Split responsibilities:
//   * CapturePlumbingPlugin: installs ECS plumbing (Event<LogEvent>, receiver, system)
//   * custom_layer(&mut World) -> Option<BoxedLayer>: builds a tracing layer from a sender
// - Ordering: Add CapturePlumbingPlugin BEFORE LogPlugin so the custom_layer closure
//   can read the sender resource inserted by the plugin.
// - Avoid duplicate logs: If you also install a global subscriber elsewhere,
//   disable LogPlugin to avoid double output.
// - ReplPlugins is unchanged. This example runs headless in the terminal.

fn main() {
    App::new()
        // Headless loop in the terminal
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
        )
        // Keyboard input is required
        .add_plugins(bevy::input::InputPlugin::default())
        // 1) Install capture plumbing before LogPlugin
        .add_plugins(bevy_repl::log_ecs::CapturePlumbingPlugin)
        // 2) Add Bevy's LogPlugin, providing our custom layer builder
        .add_plugins(bevy::log::LogPlugin {
            custom_layer: |app| bevy_repl::log_ecs::custom_layer(app),
            ..Default::default()
        })
        // 3) Add the REPL (unchanged)
        .add_plugins(bevy_repl::plugin::ReplPlugins)
        // Demo: emit some logs periodically so you can see them above the prompt
        .add_systems(Update, ticker)
        .run();
}

#[derive(Resource, Default)]
struct Tick(u32);

fn ticker(time: Res<Time>, mut acc: Local<f32>, mut tick: ResMut<Tick>) {
    *acc += time.delta_secs();
    if *acc >= 1.0 {
        *acc = 0.0;
        tick.0 += 1;
        let n = tick.0;
        match n % 5 {
            0 => info!("tick info {n}"),
            1 => warn!("tick warn {n}"),
            2 => debug!("tick debug {n}"),
            3 => error!("tick error {n}"),
            _ => trace!("tick trace {n}"),
        }
    }
}
