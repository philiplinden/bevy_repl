use bevy::prelude::*;
use bevy_repl::prelude::*;

// Define a simple command struct
#[derive(Debug, Clone, Event)]
struct SayCommand {
    message: String,
    repeat: usize,
}

// Implement ReplCommand trait with builder pattern
impl ReplCommand for SayCommand {
    fn command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true)
            )
            .arg(
                clap::Arg::new("repeat")
                    .short('r')
                    .long("repeat")
                    .help("Number of times to repeat")
                    .default_value("1")
            )
    }
    
    fn parse_from_args(args: &[&str]) -> Result<Self, clap::Error> {
        let matches = Self::command().get_matches_from(args);
        
        let message = matches.get_one::<String>("message")
            .ok_or_else(|| clap::Error::new(clap::error::ErrorKind::MissingRequiredArgument))?
            .clone();
            
        let repeat = matches.get_one::<String>("repeat")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
            
        Ok(SayCommand { message, repeat })
    }
}

// Function that handles the say command using Bevy's Trigger
fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();
    println!("Saying: {}", command.message);
    
    for i in 0..command.repeat {
        println!("  {}: {}", i + 1, command.message);
    }
}

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        MinimalPlugins,
        ReplPlugin,
    ));
    
    // Register the command with its handler
    app.repl::<SayCommand>(on_say);
    
    app.run();
}

