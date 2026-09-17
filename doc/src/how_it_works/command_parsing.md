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

## Capturing crossterm key events

The REPL captures crossterm key events and emits them as `ReplBufferEvent` after
matching the key against the keymap. If no binding matches a REPL action (Clear,
Backspace, Delete, Left, Right, Home, End, Submit command) the key is stored in
the input buffer as a character.

Input parsing is logged at the trace level as seen in the
[show_prompt_actions](https://github.com/philiplinden/bevy_repl/blob/main/examples/show_prompt_actions.rs)
example:

```
2025-08-29T02:39:41.436320Z TRACE: bevy_repl::prompt::input: Insert('h')
2025-08-29T02:39:41.606070Z TRACE: bevy_repl::prompt::input: Insert('e')
2025-08-29T02:39:42.890644Z TRACE: bevy_repl::prompt::input: Insert('l')
2025-08-29T02:39:43.059817Z TRACE: bevy_repl::prompt::input: Insert('l')
2025-08-29T02:39:43.363180Z TRACE: bevy_repl::prompt::input: Insert('o')
2025-08-29T02:45:18.595779Z TRACE: bevy_repl::prompt::input: Submit
2025-08-29T02:45:18.612872Z ERROR: bevy_repl::command::parser: Unknown command 'hello'
```

After the input parsing system, the REPL plugin clears key events and stops
keyboard input from being forwarded to Bevy when REPL is enabled. This prevents
key events from reaching game systems while typing into the prompt. The REPL
clears Crossterm key events _and_ Bevy key events spawned by `bevy_ratatui`.

Key events can be parsed before the REPL clears them by placing systems in or
before the `ReplSet::Pre` set. This is useful for wiring up keys that manage the
REPL itself. See the
[keybinds](https://github.com/philiplinden/bevy_repl/blob/main/examples/keybinds.rs)
example for a demonstration.

## Key events are not forwarded to Bevy while the REPL is enabled
All key events are cleared by the REPL when it is enabled, so they are not
forwarded to Bevy and causing unexpected behavior when typing in the prompt.
This is a tradeoff between simplicity and utility. It would be simpler to enable
raw mode and detect raw keycode commands for the toggle key, then forward the
raw inputs to Bevy as normal keycode events. However, this means that the app
input handling fundamentally changes, even when the REPL is disabled. For
development, it is more useful to have the app behave exactly as a normal
headless app when the REPL is disabled to preserve consistency in input handling
behavior.

If you really need key events or button input while the REPL is enabled, you can
place your event reader system in or before `ReplSet::Pre` in the app schedule.
This will ensure that your system is called before the REPL plugin, so keyboard
and button inputs can be read before the REPL clears them.

```rust
App::new()
    .add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0/60.0))),
        ReplPlugins,
    ))
    .add_systems(Update, your_event_reader_system.in_set(bevy_repl::ReplSet::Pre))
    .run();
```
