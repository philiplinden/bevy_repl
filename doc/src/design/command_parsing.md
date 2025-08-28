# Command Parsing

Input is parsed via `clap` commands and corresponding observer systems that execute when triggered by the REPL.

## Flow

- Define a type implementing `ReplCommand` (builder pattern) or derive it (see Derive page).
- Register the command with `.add_repl_command::<T>()`.
- Handle it with an observer: `.add_observer(on_command)`.

The REPL parses prompt input to a `T` and emits it as an event; observers run in `PostUpdate` with full ECS access.

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
