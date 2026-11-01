# How It Works

BevyREPL is read-execute-print-loop designed to be an interactive console for
the Bevy app at runtime. It provides a command line interface (CLI) to trigger
commands and inspect the app state without a window or renderer. The REPL leans
on common CLI patterns and conventions, including all features supported by
[`clap`](https://docs.rs/clap), for a familiar and intuitive user experience.

This chapter documents the design of the REPL and its components. It serves as
here as a reference for myself and for anyone who wants to understand how the
REPL works.

<!-- toc -->

## Command Parsing

Input is parsed via `clap` commands and corresponding observer systems that
execute when triggered by the REPL.

### Minimal example (builder pattern)

```rust
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

fn on_say(t: Trigger<Say>) { println!("{}", t.msg); }
```

See `examples/` for more.
