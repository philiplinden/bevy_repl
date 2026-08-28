use bevy::prelude::*;

pub mod parser;
pub mod register;

pub use parser::{
    CommandParser, ParserPlugin, TypedCommandParser, parse_input_buffer_for_commands,
};
pub use register::{ReplAppExt, register_command_in_repl};

pub type ReplResult<T> = Result<T, clap::error::Error>;

/// Trait for commands that can be registered with the REPL.
///
/// Implementors define their CLI argument schema via [`clap::Command`] and specify
/// how parsed arguments are converted into a strongly-typed Bevy [`Event`].
///
/// # The Command Execution Pipeline
///
/// When a user enters a command into the terminal, it takes the following path:
///
/// 1. **Tokenization**: `shell-words` splits the raw input string into `argv` tokens (handling quotes/escapes).
/// 2. **Parsing**: `clap` matches `argv` against [`ReplCommand::clap_command`], producing [`clap::ArgMatches`].
/// 3. **Conversion ([`ReplCommand::to_event`])**: Converts parsed [`clap::ArgMatches`] into the typed `Self` event struct.
/// 4. **Dispatch**: Bevy triggers observers listening to `Trigger<Self>` via `commands.trigger(event)`.
///
/// # Examples
///
/// ## Parameterized Command
///
/// For commands with arguments, [`to_event`](ReplCommand::to_event) extracts typed values from `matches`:
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_repl::prelude::*;
///
/// #[derive(Event, Clone, Debug)]
/// pub struct SpawnCommand {
///     pub name: String,
///     pub count: u32,
/// }
///
/// impl ReplCommand for SpawnCommand {
///     fn clap_command() -> clap::Command {
///         clap::Command::new("spawn")
///             .about("Spawns an entity")
///             .arg(clap::Arg::new("name").required(true).help("Entity name"))
///             .arg(
///                 clap::Arg::new("count")
///                     .short('c')
///                     .long("count")
///                     .default_value("1")
///                     .value_parser(clap::value_parser!(u32)),
///             )
///     }
///
///     fn to_event(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
///         let name = matches.get_one::<String>("name").unwrap().clone();
///         let count = *matches.get_one::<u32>("count").unwrap();
///         Ok(Self { name, count })
///     }
/// }
/// ```
///
/// ## Zero-Argument / Unit Command
///
/// For flag or unit commands (like `quit` or `clear`), `to_event` simply constructs `Self`:
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_repl::prelude::*;
///
/// #[derive(Event, Clone, Debug)]
/// pub struct QuitCommand;
///
/// impl ReplCommand for QuitCommand {
///     fn clap_command() -> clap::Command {
///         clap::Command::new("quit").about("Exits the application")
///     }
///
///     fn to_event(_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
///         Ok(Self)
///     }
/// }
/// ```
///
/// ## Derive Macro (`#[derive(ReplCommand)]`)
///
/// When using `bevy_repl_derive`, you can derive `ReplCommand` alongside `clap::Parser` to generate
/// `clap_command` and `to_event` automatically:
///
/// ```rust,ignore
/// #[derive(clap::Parser, ReplCommand, Event, Clone, Debug)]
/// #[command(name = "spawn", about = "Spawns an entity")]
/// pub struct SpawnCommand {
///     pub name: String,
///     #[arg(short, long, default_value_t = 1)]
///     pub count: u32,
/// }
/// ```
pub trait ReplCommand: Send + Sync + Clone + Event<Trigger<'static>: Default> + 'static {
    /// Returns the [`clap::Command`] definition for this command.
    fn clap_command() -> clap::Command;

    /// Converts parsed [`clap::ArgMatches`] into the strongly-typed Bevy event.
    ///
    /// This acts as the constructor bridging parsed CLI string values to your Rust struct.
    ///
    /// TIP: avoid this boilerplate with `#[derive(ReplCommand)` (requires the `derive` feature)
    fn to_event(matches: &clap::ArgMatches) -> Result<Self, clap::Error>;

    /// Convenience helper to parse a string slice against this command's definition.
    fn parse(args: &[&str]) -> Result<clap::ArgMatches, clap::Error>
    where
        Self: Sized,
    {
        Self::clap_command().try_get_matches_from(args)
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[derive(Debug, Clone, Event, Default)]
    #[allow(dead_code)]
    struct TestCommand {
        pub message: Option<String>,
        pub count: u32,
    }

    impl ReplCommand for TestCommand {
        fn clap_command() -> clap::Command {
            clap::Command::new("test")
                .about("A test command")
                .arg(clap::Arg::new("message").help("Message to print"))
        }

        fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
            Ok(Self {
                message: matches.get_one::<String>("message").cloned(),
                count: 0,
            })
        }
    }

    #[test]
    fn test_command_registration() {
        let cmd = TestCommand::clap_command();
        assert_eq!(cmd.get_name(), "test");
    }

    #[test]
    fn test_command_parsing_with_args() {
        let argv = shell_words::split("test hello").unwrap();
        let cmd = TestCommand::clap_command();
        let result = cmd.try_get_matches_from(&argv);
        assert!(result.is_ok(), "Should parse args successfully");
        let matches = result.unwrap();
        assert_eq!(matches.get_one::<String>("message").unwrap(), "hello");
    }

    #[test]
    fn test_command_parsing_multiple_args_fails() {
        let argv = shell_words::split("test hello world").unwrap();
        let cmd = TestCommand::clap_command();
        let result = cmd.try_get_matches_from(&argv);
        // Should fail because "world" is an extra argument
        assert!(result.is_err(), "Should fail with extra args");
    }

    #[test]
    fn test_command_parsing_no_args() {
        let argv = shell_words::split("test").unwrap();
        let cmd = TestCommand::clap_command();
        let result = cmd.try_get_matches_from(&argv);
        assert!(result.is_ok(), "Should parse command with no args");
    }

    #[test]
    fn test_shell_words_parsing() {
        use shell_words::split;

        let result = split("test hello world").unwrap();
        assert_eq!(result, vec!["test", "hello", "world"]);

        let result = split("test \"hello world\"").unwrap();
        assert_eq!(result, vec!["test", "hello world"]);

        let result = split("test hello\\ world").unwrap();
        assert_eq!(result, vec!["test", "hello world"]);
    }
}

#[cfg(test)]
mod repl_buffer_tests {
    use crate::repl::Repl;

    #[test]
    fn test_repl_buffer_insert() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        assert_eq!(repl.buffer, "hi");
        assert_eq!(repl.cursor_pos, 2);
    }

    #[test]
    fn test_repl_buffer_backspace() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        repl.backspace();
        assert_eq!(repl.buffer, "h");
        assert_eq!(repl.cursor_pos, 1);
    }

    #[test]
    fn test_repl_buffer_delete() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        // Move cursor back to position 1, then delete removes 'i'
        repl.left();
        repl.delete();
        assert_eq!(repl.buffer, "h");
    }

    #[test]
    fn test_repl_buffer_left_right() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        repl.left();
        assert_eq!(repl.cursor_pos, 1);
        repl.insert('s');
        assert_eq!(repl.buffer, "hsi");
        repl.right();
        assert_eq!(repl.cursor_pos, 3);
    }

    #[test]
    fn test_repl_buffer_home_end() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        repl.home();
        assert_eq!(repl.cursor_pos, 0);
        repl.end();
        assert_eq!(repl.cursor_pos, 2);
    }

    #[test]
    fn test_repl_buffer_clear() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        repl.clear_buffer();
        assert!(repl.buffer.is_empty());
        assert_eq!(repl.cursor_pos, 0);
    }

    #[test]
    fn test_repl_buffer_drain() {
        let mut repl = Repl::default();
        repl.insert('h');
        repl.insert('i');
        let drained = repl.drain_buffer();
        assert_eq!(drained, "hi");
        assert!(repl.buffer.is_empty());
    }

    #[test]
    fn test_repl_buffer_bounds() {
        let mut repl = Repl::default();

        repl.backspace();
        assert_eq!(repl.cursor_pos, 0);

        repl.insert('h');
        repl.right();
        assert_eq!(repl.cursor_pos, 1);
        repl.right();
        assert_eq!(repl.cursor_pos, 1);

        repl.delete();
        assert_eq!(repl.buffer, "h");
    }
}
