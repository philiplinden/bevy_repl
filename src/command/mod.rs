use anyhow::Result;
use bevy::prelude::*;

pub mod parser;
pub mod register;

pub use parser::{
    parse_input_buffer_for_commands, CommandParser, ParserPlugin, TypedCommandParser,
};
pub use register::{register_command_in_repl, ReplAppExt};

pub type ReplResult<T> = Result<T, clap::error::Error>;

/// Trait for commands that can be registered with the REPL
pub trait ReplCommand:
    Send + Sync + Clone + Event<Trigger<'static>: Default> + Default + 'static
{
    /// Returns the clap::Command definition for this command
    fn clap_command() -> clap::Command;

    /// Create the command event from parsed clap argument matches
    fn to_event(_matches: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(Self::default())
    }

    /// Parse arguments from a string slice
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
