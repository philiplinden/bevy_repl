# Writing Commands

The REPL works the same way in headless and windowed apps. In both cases you
type commands into the terminal the app was started from.

Add `bevy_repl::ReplPlugins` to your app to enable the REPL. By default, the
REPL includes the [built-in commands](default_commands.md) `quit`,
`help`, and `clear`.

Add a command to the app with `.add_repl_command::<YourCommand>()`. The command
struct must implement the `Event` and `ReplCommand` traits. When the user
enters a command, the REPL parses it with `clap` and triggers the command
struct as an event, with the command's arguments and options as its fields.

Add an observer for the command with `.add_observer(your_observer)`. The
observer receives the event as `On<YourCommand>` and can perform any action it
needs to with full ECS access. For more information about observers, see the
[Bevy examples](https://bevyengine.org/examples/ecs-entity-component-system/observers/).

There are two ways to describe a command:

- [Builder pattern](builder_pattern.md): implement `ReplCommand` by hand with
  clap's builder API.
- [Derive pattern](derive_pattern.md): derive clap's `Parser` and
  `ReplCommand` (requires the `derive` feature, on by default).
