# Vision for BevyREPL

As a developer, I see the terminal as a view to the engine running "under the
hood". The engine logs its behavior and other messages at runtime that aren't
necessarily shown to the user, and are not part of the end user's experience. In
my opinion, this means that the terminal should give _access_ to the engine
under the hood as well.

## The current experience

Bevy's input system doesn't make it easy to interact with an app with a Command
Line Interface (CLI) or command console. Out of the box, text input is handled
by a user interface and parsing the text into events or other game behavior is
left to the app developer.

1. Text input requires a windowed app with a renderer, and the text is handled
   by a GUI element, like
   [bevy-console](https://github.com/makspll/bevy-console); or
2. The default renderer is replaced by a TUI (which is just a renderer that
   happens to not leave the terminal), like
   [bevy_ratatui](https://github.com/ratatui/bevy_ratatui) and
   [bevyterm](https://github.com/Mimea005/bevyterm).
3. There is no windowing system or renderer, but then consequently no text
   input system.

The developer experience of a quake-style console is a good example of this
vision. For Bevy, however,
[bevy-console](https://github.com/NiklasEi/bevy_console) is built upon
[egui](https://github.com/emilk/egui) and therefore only available in windowed
applications. Not to mention that Egui is a large dependency to add, especially
for an app that is not using Egui for anyhing else.

Interaction with Bevy from the terminal usually calls for a TUI (terminal user
interface) renderer like [bevy_ratatui](https://github.com/ratatui/bevy_ratatui)
or [bevyterm](https://github.com/Mimea005/bevyterm), which is a lot of overhead
for a developer tool, especially when the app doesn't use the terminal for
anything in the app nominally. On top of that, the developer would have to route
logs to the TUI, add custom input areas and command parsing, and so on.

## The ideal experience

An ideal experience would be a terminal-based REPL (Read-Eval-Print Loop) that
is available out of the box with no configuration or setup beyond adding a new
plugin to the app. It should be simple to add custom commands and arguments to
the REPL without much boilerplate, and should feel familiar to setting up any
other conventional CLI.

Nice-to-haves would be to see the nominal log messages from the engine alongside
the REPL output, having the REPL output and logs persist in the terminal stdout
after the app exits, and having command history or tab completion for commands.

## Requirements

**The BevyREPL plugin should...**
1. provide a developer interface for adding custom commands and arguments to the
   REPL that feels like building a standard CLI;
2. print to stdout, rather than a separate TUI screen that disappears when the
   app exits;
3. not conflict with nominal logging or printing to the terminal stdout;
4. allow the user to type in commands directly into the terminal without
   triggering key presses in the app itself at runtime;
6. have the same behavior whether the app has a window/rendering or not;
7. have a minimal footprint and not require any large additional dependencies;
8. not require disabling Bevy features or changing the base setup (i.e., the
   behavior should be added with the plugin alone, and all other setup would
   remain the same, so no modifications of the `DefaultPlugins` or anything like
   that)
9. simple and direct integration with Bevy ECS.
