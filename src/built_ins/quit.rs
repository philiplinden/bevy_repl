use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<QuitCommand>();
    app.add_observer(on_quit);
}

#[derive(Event, Clone, Default)]
struct QuitCommand;

impl crate::command::ReplCommand for QuitCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("quit")
            .about("Exits the app gracefully")
    }
}

fn on_quit(_trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    exit.write(AppExit::Success);
}
