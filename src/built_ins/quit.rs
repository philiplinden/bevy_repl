use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_repl_command::<QuitCommand>();
    app.add_observer(on_quit);
}

#[derive(Event, Clone)]
struct QuitCommand {
    verbose: bool,
}

impl ReplCommand for QuitCommand {
    fn command() -> clap::Command {
        clap::Command::new("quit")
            .about("Exits the app gracefully")
            .arg(
                clap::Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enables verbose output")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn from_matches(matches: clap::ArgMatches) -> Self {
        let verbose = matches.get_flag("verbose");
        QuitCommand { verbose }
    }
}

fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    let cmd = trigger.event();
    if cmd.verbose {
        info!("Quitting (verbose)...");
    }
    exit.write(AppExit::Success);
}
