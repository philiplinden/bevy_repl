# Vision for BevyREPL

_Archived 2026-11-02 from `design/vision.md`. Written before deciding on a TUI, runtime toggling and a `custom_layer` for logs. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

Out of the box, Bevy captures keypresses as events. This leaves developers to pick one of three setups, and each one costs something:

1. Text input requires a windowed app with a renderer, and the text is handled
   by a GUI element, like
   [bevy-console](https://github.com/RichoDemus/bevy-console). That console is
   built with [egui](https://github.com/emilk/egui), which is a large
   dependency to add, especially for an app that is not using egui for
   anything else;
2. The default renderer is replaced by a TUI (which is just a renderer that
   happens to not leave the terminal), like
   [bevy_ratatui](https://github.com/ratatui/bevy_ratatui) and
   [bevyterm](https://github.com/Mimea005/bevyterm). That's a lot of overhead
   for a developer tool, especially when the app doesn't use the terminal for
   anything else. The developer still has to route logs to the TUI and add
   custom input areas and command parsing, and then logs disappear when the
   app closes; or
3. There is no windowing system or renderer (the app runs headless) and a
   custom user input solution has to be developed and maintained for headless
   mode separately from the normal input capture systems.

An ideal experience would feel like a quake-style console, but as a terminal-based
REPL that is available out of the box with no configuration or setup beyond adding a new
plugin to the app. It should be simple to add custom commands and arguments to
the REPL without much boilerplate, and should feel familiar to setting up any
other conventional CLI.

**The BevyREPL plugin should...**

1. provide a developer interface for adding custom commands and arguments to the
   REPL that feels like building a standard CLI, where each command is a Bevy
   event handled by observers;
2. print to the terminal, rather than a separate TUI screen that disappears
   when the app exits;
3. not conflict with nominal logging or printing to the terminal;
4. allow the user to type in commands directly into the terminal without
   triggering key presses in the app itself at runtime;
5. have the same behavior whether the app has a window/rendering or not;
6. have a minimal footprint and not require any large additional dependencies;
7. not require disabling Bevy features or changing the base setup (i.e., the
   behavior should be added with the plugin alone, and all other setup would
   remain the same, so no modifications of the `DefaultPlugins` or anything like
   that);
8. always restore the terminal when the app exits, including on Ctrl+C or a
   panic.

**Not planned for 1.0:** runtime toggling, command history, tab completion, custom
prompts, and configurable keybinds.
