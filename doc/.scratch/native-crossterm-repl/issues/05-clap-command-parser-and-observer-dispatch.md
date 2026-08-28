# Clap Command Parser and Observer Dispatch

Type: grilling
Status: resolved

## Question

How should the command registration, shell tokenization, clap error reporting, and Bevy Observer dispatch pipeline be structured without dependencies on `bevy_ratatui`?

Specifically:
1. How does `shell-words` tokenize the submitted buffer string into `argv`?
2. How are clap errors (like `--help` or invalid arguments) formatted and output to the terminal scroll region?
3. How is `Commands::trigger(event)` invoked generically for any registered `ReplCommand` implementing Bevy's `Event` trait?

## Answer

### 1. Robust `ReplCommand` Trait Specification
To avoid silent argument-dropping bugs, `to_event` is a **required method** without a misleading default fallback, eliminating the artificial `Default` trait bound:

```rust
pub trait ReplCommand: Send + Sync + Clone + Event + 'static {
    /// Returns the clap::Command definition
    fn clap_command() -> clap::Command;

    /// Converts parsed ArgMatches into the typed Event struct.
    /// Required explicitly to ensure argument mappings are never silently dropped.
    fn to_event(matches: &clap::ArgMatches) -> Result<Self, clap::Error>;
}
```

- **Manual Implementations**: Implementers explicitly map `matches` to `Self`.
- **Derive Macro Support (`bevy_repl_derive`)**: Users can derive `#[derive(ReplCommand, clap::Parser, Event)]` to auto-generate `clap_command()` and `to_event()` via `clap::FromArgMatches`.

### 2. Tokenization & Dispatch Pipeline
1. In `parse_input_buffer_for_commands`, use `shell_words::split(&input)` to parse shell tokens, respecting quotes and backslash escapes.
2. Look up the command name or alias `&argv[0]` in `repl.commands: HashMap<String, Box<dyn CommandParser>>`.
3. If not found, print `Unknown command '...'. Type 'help' to see available commands.` via `repl_println!`.

### 3. Generic Observer Dispatch (`Commands::trigger`)
`TypedCommandParser<C: ReplCommand>` encapsulates `clap` parsing and dispatches the resulting typed event directly to Bevy Observers:

```rust
pub struct TypedCommandParser<C: ReplCommand> {
    _phantom: std::marker::PhantomData<C>,
}

impl<C: ReplCommand> CommandParser for TypedCommandParser<C> {
    fn parse_and_trigger(&self, input: &str, commands: &mut Commands) -> bool {
        let Ok(argv) = shell_words::split(input) else { return false };
        let cmd = C::clap_command();

        match cmd.try_get_matches_from(&argv) {
            Ok(matches) => {
                match C::to_event(&matches) {
                    Ok(event) => {
                        commands.trigger(event);
                    }
                    Err(err) => {
                        for line in format!("{}", err).lines() {
                            repl_println!("{}", line);
                        }
                    }
                }
                true
            }
            Err(clap_err) => {
                // Formats --help, --version, or argument errors with CRLF
                for line in format!("{}", clap_err).lines() {
                    repl_println!("{}", line);
                }
                true
            }
        }
    }
}
```

### 4. Zero-Boilerplate App Extension
Provide `app.add_repl_command::<C>()` via `ReplAppExt` to register the parser in `Repl.commands` and enable `app.add_observer(|trigger: Trigger<C>| { ... })`.
