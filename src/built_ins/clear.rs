use bevy::prelude::*;
use crate::prelude::*;
use crate::prompt::renderer::stdout::StdoutTerminalContext;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<ClearCommand>();
    app.add_observer(on_clear);
}

#[derive(Event, Clone, Default)]
struct ClearCommand;

impl crate::command::ReplCommand for ClearCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("clear").about("Clears previous outputs from the REPL")
    }
}

fn on_clear(_trigger: Trigger<ClearCommand>, terminal: Option<ResMut<StdoutTerminalContext>>) {
    if let Some(mut term) = terminal {
        if let Err(e) = term.clear() {
            error!("Failed to clear terminal: {}", e);
        }
    } else {
        // No stdout terminal context present (likely in alt-screen). No-op; next frame redraws UI.
        info!("Clear requested, but no StdoutTerminalContext present; skipping.");
    }
}
