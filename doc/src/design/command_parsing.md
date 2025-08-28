# Command Parsing

Input is parsed via `clap` commands and corresponding observer systems that
execute when triggered by the REPL.

<!-- toc -->

## Minimal example (builder pattern)

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
