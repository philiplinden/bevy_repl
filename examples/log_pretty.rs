use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Resource)]
struct SpamTimer(Timer);

fn setup(mut commands: Commands) {
    commands.insert_resource(SpamTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    bevy_repl::repl_println!("\nBevy log routing example");
    bevy_repl::repl_println!(
        "Logs emitted by Bevy/tracing are captured and printed above the REPL prompt."
    );
    bevy_repl::repl_println!("Type `quit` to exit.");
}

fn spam_logs(mut timer: ResMut<SpamTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        tracing::error!("Example error log");
        tracing::warn!("Example warn log");
        tracing::info!("Example info log");
        tracing::debug!(delta = ?time.delta(), "Example debug log");
        tracing::trace!("Example trace log");
    }
}

fn main() {
    // Install a global fmt layer that writes logs directly to the REPL printer,
    // preserving colors and formatting. Do this BEFORE adding DefaultPlugins.
    tracing_to_repl_fmt_with_level(bevy::log::Level::DEBUG);

    App::new()
        .add_plugins((
            // Disable stdout logger to avoid duplicate output; our fmt layer handles printing
            DefaultPlugins.build().disable::<bevy::log::LogPlugin>(),
            // Enable the REPL (pretty recommended for the prompt scroll region)
            ReplPlugins.set(PromptPlugin::pretty()),
        ))
        .add_systems(Startup, setup)
        // Emit logs at various levels
        .add_systems(Update, spam_logs)
        .run();
}
