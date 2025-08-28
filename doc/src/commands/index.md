# Usage

The REPL is designed to be used in headless mode, but it can be used in windowed
mode too through the terminal while the app is running.

_It is not possible to toggle the REPL on and off at runtime (yet!)._

Add `bevy_repl::ReplPlugins` to your app to enable the REPL and print logs to
stdout. By default, the REPL includes a `quit` command to terminate the app.

Add a command to the app with `.add_repl_command<YourReplCommand>()`. The
command struct must implement the `Event` and `ReplCommand` traits. When the
user enters a command, the REPL parses it with `clap` and emits an event with the
command's arguments and options as the fields of the event.

Add an observer for the command with `.add_observer(your_observer)`. The
observer is a one-shot system that receives the event and can perform any action
it needs to with full ECS access, and is a feature included in Bevy. For more
information about observers, see: [Bevy examples](https://bevyengine.org/examples/ecs-entity-component-system/observers/).
