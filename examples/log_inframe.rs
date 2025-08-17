use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::log_ecs::LogCaptureConfig;
use std::time::Duration;

// Demonstrates REPL-orchestrated in-frame logging.
// The REPL installs a global tracing subscriber (via LogCaptureConfig)
// and renders recent logs inside the ratatui frame above the prompt.
//
// Notes:
// - This approach does NOT use Bevy's LogPlugin custom_layer.
// - Set `init_subscriber = true` so REPL installs the subscriber.
// - The minimal renderer will draw recent LogBuffer lines automatically.
// - If you also add Bevy's LogPlugin, disable its stdout logging to avoid duplicates.

fn main() {
    App::new()
        // Headless loop
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
        )
        // Keyboard input
        .add_plugins(bevy::input::InputPlugin::default())
        // REPL logs config: REPL installs a global subscriber
        .insert_resource(LogCaptureConfig {
            level: bevy::log::Level::INFO,
            capacity: 512,
            init_subscriber: true,
        })
        // Run the REPL
        .add_plugins(bevy_repl::plugin::ReplPlugins)
        // Emit some logs periodically to see them above the prompt
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
            0 => info!("inframe info {n}"),
            1 => warn!("inframe warn {n}"),
            2 => debug!("inframe debug {n}"),
            3 => error!("inframe error {n}"),
            _ => trace!("inframe trace {n}"),
        }
    }
}
