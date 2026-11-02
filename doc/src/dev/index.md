# How It Works

BevyREPL is a read-eval-print loop designed to be an interactive console for
the Bevy app at runtime. It provides a command line interface (CLI) to trigger
commands and inspect the app state without a window or renderer. The REPL leans
on common CLI patterns and conventions, including all features supported by
[`clap`](https://docs.rs/clap), for a familiar and intuitive user experience.

This chapter documents the design of the REPL and its components. It serves as a
reference for myself and for anyone who wants to understand how the
REPL works.

<!-- toc -->

## Command Parsing

When the user presses Enter, the REPL:

1. splits the input line into shell-style `argv` tokens with `shell-words`;
2. matches the tokens against each registered command's `clap::Command`;
3. converts the matches into the command struct with `ReplCommand::to_event`;
4. triggers the command struct as an event, which runs its observers.

### Minimal example (builder pattern)

```rust,ignore
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(Debug, Clone, Event, Default)]
struct Say { msg: String }

impl ReplCommand for Say {
    fn clap_command() -> clap::Command {
        clap::Command::new("say").arg(clap::Arg::new("msg").required(true))
    }
    fn to_event(m: &clap::ArgMatches) -> ReplResult<Self> {
        Ok(Say { msg: m.get_one::<String>("msg").unwrap().clone() })
    }
}

fn on_say(t: On<Say>) { repl_println!("{}", t.event().msg); }
```

See [`examples/`](https://github.com/philiplinden/bevy_repl/tree/main/examples)
for more.
