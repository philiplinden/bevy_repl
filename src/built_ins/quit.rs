use crate::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<QuitCommand>();
    app.add_observer(on_quit);
}

#[derive(Event, Clone, Default)]
struct QuitCommand;

impl crate::command::ReplCommand for QuitCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("quit")
            .visible_alias("q")
            .visible_alias("exit")
            .about("Exits the app gracefully")
    }
}

fn on_quit(_trigger: On<QuitCommand>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
