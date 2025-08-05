use clap::{Command, Arg};

fn dev_cli() {
    let mut cmd = Command::new("say");
    cmd.arg(Arg::new("text").help("The text to say"));

    let matches = cmd.get_matches();
    
}
