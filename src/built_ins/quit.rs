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

impl clap::FromArgMatches for QuitCommand {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::error::Error> {
        Ok(QuitCommand {
            verbose: matches.get_flag("verbose"),
        })
    }
    
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::error::Error> {
        self.verbose = matches.get_flag("verbose");
        Ok(())
    }
}

impl ReplCommand for QuitCommand {
    fn clap_command() -> clap::Command {
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
}

fn on_quit(trigger: Trigger<QuitCommand>, mut exit: EventWriter<AppExit>) {
    let cmd = trigger.event();
    if cmd.verbose {
        info!("Quitting (verbose)...");
    }
    exit.write(AppExit::Success);
}
