use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    let mut app = App::new();

    // Run in headless mode at 60 fps
    app.add_plugins((
        MinimalPlugins,
        bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )
    ));

    // Add REPL with custom commands
    app.add_plugins((
        ReplPlugin,
        ReplDefaultCommandsPlugin,
    ))
    .repl::<SayCommand>(on_say);

    app.run();
}

/// Example command using clap's derive pattern
#[derive(ReplParser)]
#[command(name = "say", about = "Say something")]
struct SayCommand {
    #[arg(short, long)]
    message: String,
}

/// Observer function for hello
fn on_say(trigger: Trigger<SayCommand>) {
    println!("{}", trigger.message);
}
