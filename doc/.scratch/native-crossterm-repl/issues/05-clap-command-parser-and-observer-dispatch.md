# Clap Command Parser and Observer Dispatch

Type: grilling
Status: open
Blocked by: 02

## Question

How should the command registration, shell tokenization, clap error reporting, and Bevy Observer dispatch pipeline be structured without dependencies on `bevy_ratatui`?

Specifically:
1. How does `shell-words` tokenize the submitted buffer string into `argv`?
2. How are clap errors (like `--help` or invalid arguments) formatted and output to the terminal scroll region?
3. How is `Commands::trigger(event)` invoked generically for any registered `ReplCommand` implementing Bevy's `Event` trait?
